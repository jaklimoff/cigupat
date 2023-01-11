use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Choices {
    pub text: String,
    pub index: u32,
    pub logprobs: Option<String>,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub model: String,
    pub created: u64,
    pub choices: Vec<Choices>,
    pub usage: Usage,
}