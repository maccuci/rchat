use openai_api_rs::v1::api::Client;
use std::env;
use std::error::Error;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest, ChatCompletionResponse};
use openai_api_rs::v1::common::GPT3_5_TURBO;

pub fn invoke() -> Client {
    let client = Client::new(env::var("OPENAI_API").unwrap().to_string());
    client
}

pub fn request(client: Client, content: String) -> Result<(), Box<dyn Error>> {
    let req = ChatCompletionRequest::new(
        GPT3_5_TURBO.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content,
            name: None,
            function_call: None,
        }],
    );
    let result: ChatCompletionResponse = client.chat_completion(req)?;
    let msg: Option<String> = result.choices[0].message.content.clone();

    println!("{:?}", msg);
    Ok(())
}