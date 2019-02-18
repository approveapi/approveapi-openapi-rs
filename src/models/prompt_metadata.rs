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
pub struct PromptMetadata {
    /// The date/time of the action.
    #[serde(rename = "time")]
    pub time: Option<String>,
    /// The operating system initiating the action, i.e. Mac OS X.
    #[serde(rename = "operating_system")]
    pub operating_system: Option<String>,
    /// The IP address of the computer initiating the action.
    #[serde(rename = "ip_address")]
    pub ip_address: Option<String>,
    /// The physical location, like Oakland, CA, of the action.
    #[serde(rename = "location")]
    pub location: Option<String>,
    /// The web browser initiating the action, i.e. Chrome.
    #[serde(rename = "browser")]
    pub browser: Option<String>,
}

impl PromptMetadata {
    pub fn new() -> PromptMetadata {
        PromptMetadata {
            time: None,
            operating_system: None,
            ip_address: None,
            location: None,
            browser: None,
        }
    }
}