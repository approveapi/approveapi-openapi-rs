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
pub struct PromptAnswer {
    /// The user's answer to whether or not they approve this prompt.
    #[serde(rename = "result")]
    pub result: bool,
    /// The unix timestamp when the user answered the prompt.
    #[serde(rename = "time")]
    pub time: f32,
    #[serde(rename = "metadata")]
    pub metadata: Option<::models::AnswerMetadata>,
}

impl PromptAnswer {
    pub fn new(result: bool, time: f32) -> PromptAnswer {
        PromptAnswer {
            result: result,
            time: time,
            metadata: None,
        }
    }
}
