/*
 * ApproveAPISwagger
 *
 * The simple API to request a user's approval on anything via email + sms.
 *
 * OpenAPI spec version: 1.0.1
 * Contact: dev@approveapi.com
 * Generated by: https://openapi-generator.tech
 */

use hyper;
use std::collections::HashMap;

pub struct Configuration<C: hyper::client::connect::Connect> {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: hyper::client::Client<C>,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
    // TODO: take an oauth2 token source, similar to the go one
}

pub type BasicAuth = (String, Option<String>);

pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl<C: hyper::client::connect::Connect> Configuration<C> {
    pub fn new(client: hyper::client::Client<C>) -> Configuration<C> {
        Configuration {
            base_path: "https://approve.sh".to_owned(),
            user_agent: Some("OpenAPI-Generator/1.0.1/rust".to_owned()),
            client: client,
            basic_auth: None,
            oauth_access_token: None,
            api_key: None,
        }
    }
}
