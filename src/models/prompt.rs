/*
 * ApproveAPISwagger
 *
 * The simple API to request a user's approval on anything via email + sms.
 *
 * OpenAPI spec version: 1.0.1
 * Contact: dev@approveapi.com
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Prompt {
    #[serde(rename = "answer")]
    pub answer: Option<::models::PromptAnswer>,
    /// The unix timestamp when this prompt was sent.
    #[serde(rename = "sent_at")]
    pub sent_at: f32,
    /// Whether the prompt can still be answered.
    #[serde(rename = "is_expired")]
    pub is_expired: bool,
    /// A unique id for this prompt.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metadata")]
    pub metadata: Option<::models::PromptMetadata>,
}

impl Prompt {
    pub fn new(sent_at: f32, is_expired: bool, id: String) -> Prompt {
        Prompt {
            answer: None,
            sent_at: sent_at,
            is_expired: is_expired,
            id: id,
            metadata: None,
        }
    }
}
