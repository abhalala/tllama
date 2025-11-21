use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedModelInfo {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub developer: String,
    pub short_description: String,
    pub full_description: String,
    pub requires_pro: bool,
    pub premium: bool,
}

pub struct ModelScraper {
    client: reqwest::Client,
    cookies: String,
}

impl ModelScraper {
    pub fn new(cookies: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            cookies,
        }
    }

    async fn get_chunk_urls_from_homepage(
        &self,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get("https://t3.chat/")
            .header("Cookie", &self.cookies)
            .send()
            .await?;
        let html = response.text().await?;
        let script_regex =
            Regex::new(r#"<script[^>]+src="(/_next/static/chunks/[a-f0-9]+\.js[^"]*)"#)?;
        let mut chunk_urls = Vec::new();
        for capture in script_regex.captures_iter(&html) {
            let chunk_path = capture.get(1).unwrap().as_str();
            chunk_urls.push(format!("https://t3.chat{}", chunk_path));
        }
        if chunk_urls.is_empty() {
            return Err("Could not find any chunk URLs in homepage".into());
        }
        Ok(chunk_urls)
    }

    async fn parse_models_from_chunk(
        &self,
        chunk_url: &str,
    ) -> Result<Vec<ScrapedModelInfo>, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(chunk_url)
            .header("Cookie", &self.cookies)
            .header("Referer", "https://t3.chat/")
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await?;
        let js_content = response.text().await?;
        let model_list_regex = Regex::new(r#"let\s+\w+\s*=\s*\[((?:"[^"]+",?\s*)+)\]"#)?;
        let mut model_ids = Vec::new();
        if let Some(captures) = model_list_regex.captures(&js_content) {
            let models_str = captures.get(1).unwrap().as_str();
            let model_regex = Regex::new(r#""([^"]+)""#)?;
            for capture in model_regex.captures_iter(models_str) {
                model_ids.push(capture.get(1).unwrap().as_str().to_string());
            }
        }
        if model_ids.is_empty() {
            return Ok(Vec::new());
        }
        let mut models = Vec::new();
        for model_id in &model_ids {
            let pattern = format!(
                r#"(?s)"{}":\s*\{{.*?id:\s*"([^"]+)"(?s).*?name:\s*"([^"]+)"(?s).*?provider:\s*"([^"]+)"(?s).*?developer:\s*"([^"]+)"(?s).*?shortDescription:\s*"([^"]*)"(?:.*?fullDescription:\s*"([^"]*)")?"#,
                regex::escape(model_id)
            );
            if let Ok(model_regex) = Regex::new(&pattern) {
                if let Some(capture) = model_regex.captures(&js_content) {
                    let model = ScrapedModelInfo {
                        id: capture.get(1).unwrap().as_str().to_string(),
                        name: capture.get(2).unwrap().as_str().to_string(),
                        provider: capture.get(3).unwrap().as_str().to_string(),
                        developer: capture.get(4).unwrap().as_str().to_string(),
                        short_description: capture.get(5).unwrap().as_str().to_string(),
                        full_description: capture
                            .get(6)
                            .map_or(String::new(), |m| m.as_str().to_string()),
                        requires_pro: false,
                        premium: false,
                    };
                    models.push(model);
                    continue;
                }
            }
            models.push(ScrapedModelInfo {
                id: model_id.clone(),
                name: model_id.to_uppercase(),
                provider: "Unknown".to_string(),
                developer: "Unknown".to_string(),
                short_description: format!("{} model", model_id),
                full_description: String::new(),
                requires_pro: false,
                premium: false,
            });
        }
        Ok(models)
    }

    pub async fn fetch_models(&self) -> Result<Vec<ScrapedModelInfo>, Box<dyn std::error::Error>> {
        match self.fetch_models_dynamically().await {
            Ok(models) => Ok(models),
            Err(_) => self.get_fallback_models(),
        }
    }

    async fn fetch_models_dynamically(&self) -> Result<Vec<ScrapedModelInfo>, Box<dyn std::error::Error>> {
        let known_chunks = vec!["https://t3.chat/_next/static/chunks/3af0bf4d01fe7216.js"];
        for chunk_url in known_chunks {
            if let Ok(models) = self.parse_models_from_chunk(chunk_url).await {
                if models.len() > 10 {
                    return Ok(models);
                }
            }
        }
        let chunk_urls = self.get_chunk_urls_from_homepage().await?;
        for chunk_url in chunk_urls {
            if let Ok(models) = self.parse_models_from_chunk(&chunk_url).await {
                if models.len() > 10 {
                    return Ok(models);
                }
            }
        }
        Err("Could not find model definitions in any chunk".into())
    }

    fn get_fallback_models(&self) -> Result<Vec<ScrapedModelInfo>, Box<dyn std::error::Error>> {
        let models = vec![
            ScrapedModelInfo {
                id: "gemini-2.5-flash".to_string(),
                name: "gemini-2.5-flash".to_string(),
                provider: "Google".to_string(),
                developer: "Google".to_string(),
                short_description: "Google's state of the art fast model".to_string(),
                full_description: "".to_string(),
                requires_pro: false,
                premium: false,
            },
            // ... (add other fallback models as needed)
        ];
        Ok(models)
    }
}
