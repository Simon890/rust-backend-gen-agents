use crate::models::general::llm::{Message, ChatCompletion, APIResponse};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

/// Call Large language model
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    let api_key: String = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in .env");
    let api_org: String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in .env");

    let url: &str = "https://api.openai.com/v1/chat/completions";

    let mut headers = HeaderMap::new();

    headers.insert(
        "authorization", HeaderValue::from_str(&format!("Bearer {}", api_key))
        .map_err(|e| -> Box<dyn std::error::Error + Send>{Box::new(e)})?
    );
    headers.insert(
        "OpenAI-Organization", HeaderValue::from_str(api_org.as_str())
        .map_err(|e| -> Box<dyn std::error::Error + Send>{Box::new(e)})?
    );

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send>{Box::new(e)})?;
    
    let chat_completion: ChatCompletion = ChatCompletion { 
        model: "o4-mini".to_string(), 
        messages, 
        temperature: 1.0
    };

    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send>{Box::new(e)})?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send>{Box::new(e)})?;

    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test. Give me a short response".to_string()
        };
        let res = call_gpt(vec![message]).await;
        if let Ok(res_str) = res {
            dbg!(res_str);
            assert!(true)
        } else if let Err(reason) = res {
            dbg!(reason.as_ref());
            assert!(false)
        }
    }
}