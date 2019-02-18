/*
 * ApproveAPISwagger
 *
 * The simple API to request a user's approval on anything via email + sms.
 *
 * OpenAPI spec version: 1.0.0
 * Contact: dev@approveapi.com
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    /// A human readable API error message.
    #[serde(rename = "error")]
    pub error: String,
}

impl Error {
    pub fn new(error: String) -> Error {
        Error {
            error: error,
        }
    }
}
