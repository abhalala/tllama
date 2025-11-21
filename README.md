# TLlama - Ollama API Replicator for T3 Chat

TLlama replicates the Ollama API one-to-one, but uses T3 Chat as the backend. Think of it like Wine/Proton for Windows apps, but for Ollama API → T3 Chat.

## Features

- ✅ Complete Ollama API compatibility
- ✅ Uses T3 Chat backend (50+ models)
- ✅ Same endpoints as Ollama
- ✅ Works with existing Ollama clients
- ✅ Docker support

## Setup

1. Get your T3 Chat credentials:
   - Go to t3.chat in browser
   - Open DevTools (F12) → Application → Cookies
   - Copy full cookie string and convex-session-id

2. Create `.env` file:
```bash
cp .env.example .env
# Edit .env with your credentials
```

3. Run:
```bash
# Using Cargo
cargo run

# Using Docker
docker-compose up
```

## Usage

Works exactly like Ollama:

```bash
# Generate
curl http://localhost:11434/api/generate -d '{
  "model": "claude-3.7",
  "prompt": "Why is the sky blue?"
}'

# Chat
curl http://localhost:11434/api/chat -d '{
  "model": "gpt-4o",
  "messages": [
    {"role": "user", "content": "Hello!"}
  ]
}'

# List models
curl http://localhost:11434/api/tags
```

## Available Models (Now fetched live!)

All T3 Chat models work:
- claude-3.7, claude-4-opus
- gpt-4o, gpt-4o-mini
- gemini-2.0-flash
- deepseek-v3
- llama-3.3-70b
- And 40+ more!

## License

MIT
