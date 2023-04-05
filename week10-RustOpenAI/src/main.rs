// Chatbot written in Rust
// Calls OpenAI API to generate responses

use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::io::{self, BufRead, Write};

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    prompt: String,
    temperature: f32,
    max_tokens: u32,
    top_p: f32,
    frequency_penalty: f32,
    presence_penalty: f32,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let url = "https://api.openai.com/v1/completions";

    loop {
        print!("You: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input)?;
        let prompt = format!("User: {}", input.trim());

        let json = json!({
            "model": "text-davinci-003",
            "prompt": prompt,
            "temperature": 0.3,
            "max_tokens": 256,
            "top_p": 1,
            "frequency_penalty": 0,
            "presence_penalty": 0
        });


        let response = client
            .post(url)
            .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&json)
            .send()
            .await?;

        let body: Value = serde_json::from_str(&response.text().await?)?;
        let choices = body["choices"].as_array().unwrap();
        let text = choices[0]["text"].as_str().unwrap();

        println!("{}", text);
    }
}