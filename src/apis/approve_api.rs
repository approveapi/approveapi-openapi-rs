/*
 * ApproveAPISwagger
 *
 * The simple API to request a user's approval on anything via email + sms.
 *
 * OpenAPI spec version: 1.0.1
 * Contact: dev@approveapi.com
 * Generated by: https://openapi-generator.tech
 */

use std::sync::Arc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct ApproveApiClient<C: hyper::client::connect::Connect + 'static> {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect + 'static> ApproveApiClient<C> {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> ApproveApiClient<C> {
        ApproveApiClient {
            configuration: configuration,
        }
    }
}

pub trait ApproveApi {
    fn create_prompt(&self, create_prompt_request: ::models::CreatePromptRequest) -> Box<Future<Item = ::models::Prompt, Error = Error<serde_json::Value>> + Send>;
    fn get_prompt(&self, id: &str, long_poll: bool) -> Box<Future<Item = ::models::Prompt, Error = Error<serde_json::Value>> + Send>;
    fn get_prompt_status(&self, id: &str) -> Box<Future<Item = ::models::PromptStatus, Error = Error<serde_json::Value>> + Send>;
}


impl<C: hyper::client::connect::Connect + 'static>ApproveApi for ApproveApiClient<C> {
    fn create_prompt(&self, create_prompt_request: ::models::CreatePromptRequest) -> Box<Future<Item = ::models::Prompt, Error = Error<serde_json::Value>> + Send> {
        __internal_request::Request::new(hyper::Method::POST, "/prompt".to_string())
            .with_auth(__internal_request::Auth::Basic)
            .with_body_param(create_prompt_request)
            .execute(self.configuration.borrow())
    }

    fn get_prompt(&self, id: &str, long_poll: bool) -> Box<Future<Item = ::models::Prompt, Error = Error<serde_json::Value>> + Send> {
        __internal_request::Request::new(hyper::Method::GET, "/prompt/{id}".to_string())
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("long_poll".to_string(), long_poll.to_string())
            .with_path_param("id".to_string(), id.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_prompt_status(&self, id: &str) -> Box<Future<Item = ::models::PromptStatus, Error = Error<serde_json::Value>> + Send> {
        __internal_request::Request::new(hyper::Method::GET, "/prompt/{id}/status".to_string())
            .with_path_param("id".to_string(), id.to_string())
            .execute(self.configuration.borrow())
    }

}
