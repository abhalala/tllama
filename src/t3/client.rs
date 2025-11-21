use anyhow::Result;
use t3router::t3::{
    client::Client, config::Config as T3Config, message::Message as T3Message,
};

pub struct T3Client {
    client: Client,
}

impl T3Client {
    pub async fn new(cookies: String, session_id: String) -> Result<Self> {
        let client = Client::new(cookies, session_id);
        client.init().await?;
        Ok(Self { client })
    }

    #[cfg(test)]
    pub fn new_uninitialized(cookies: String, session_id: String) -> Self {
        let client = Client::new(cookies, session_id);
        Self { client }
    }

    pub async fn send_message(
        &mut self,
        model: &str,
        message: T3Message,
    ) -> Result<String> {
        let config = T3Config::new();
        let response = self.client.send(model, Some(message), Some(config)).await?;
        Ok(response.content)
    }

    pub fn get_messages(&self) -> &[T3Message] {
        self.client.get_messages()
    }

    pub fn append_message(&mut self, message: T3Message) {
        self.client.append_message(message);
    }

    pub fn clear_messages(&mut self) {
        self.client.clear_messages();
    }
}
