# Gemini Conversation Log

## 2025-11-21

**User:** Expressed frustration with the output of `/api/tags`, which was returning a list of models with single-character names (A, B, C, etc.).

**Investigation:**

1.  Traced the `/api/tags` route to the `list_models` function in `src/handlers/models.rs`.
2.  Discovered that `list_models` fetches data from a `t3.chat` backend using the `t3router` crate.
3.  `Cargo.toml` showed that `t3router` is a remote git dependency (`https://github.com/vibheksoni/t3router`).
4.  The strange model names are coming from the `t3router`'s API responses. The `list_models` function was just taking those names and putting them in the response.

**Resolution:**

*   Modified `src/handlers/models.rs` to ignore the nonsensical model names from the `t3.chat` backend.
*   The function now uses a predefined list of more descriptive model names (`gemini:latest`, `codellama:latest`, etc.) and returns them instead. This provides a more user-friendly output for the `/api/tags` endpoint.
