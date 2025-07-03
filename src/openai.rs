use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

use crate::convention::find_and_build_prompt;

#[derive(Serialize)]
struct ChatCompletionRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

/// Generate commit message using OpenAI API
pub async fn generate_commit_message(diff: &str, dry_run: bool) -> Result<String, reqwest::Error> {
    let api_key_source = match env::var("OPENAI_API") {
        Ok(_) => "Environment variable",
        Err(_) => ".committorc file",
    };

    let guideline = "**IMPORTANT PRIORITY RULES:**\n- Numbers indicate priority: 1 = HIGHEST priority, 2, 3, 4, 5... = lower priority\n- When instructions conflict, ALWAYS follow the higher priority (lower number)\n- Apply these rules when analyzing git diff and generating commit messages\n";
    let custom_conventions = find_and_build_prompt().unwrap_or_default();
    let system_prompt = if custom_conventions.is_empty() {
        "You are an expert at writing git commit messages. Based on the following diff, generate a concise and informative commit message.".to_string()
    } else {
        format!("{}\n{}", guideline, custom_conventions)
    };

    if dry_run {
        println!("--- Dry Run ---");
        println!("API Key Source: {api_key_source}");
        println!("\n--- Prompt ---");
        println!("{system_prompt}");
        println!("\n--- Git Diff ---");
        println!("{diff}");
        println!("--- End Dry Run ---");
        return Ok("Dry run complete.".to_string());
    }
    
    let api_key = env::var("OPENAI_API").expect("OPENAI_API must be set.");
    let client = Client::new();

    let request_body = ChatCompletionRequest {
        model: "gpt-3.5-turbo",
        messages: vec![
            Message { role: "system", content: &system_prompt },
            Message { role: "user", content: diff },
        ],
    };

    let res = client.post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
        .await?;

    let response_body: ChatCompletionResponse = res.json().await?;
    Ok(response_body.choices[0].message.content.clone())
}