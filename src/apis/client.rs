use std::sync::Arc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::connect::Connect + 'static> {
    configuration: Arc<Configuration<C>>,
    approve_api: Box<::apis::ApproveApi>,
}

impl<C: hyper::client::connect::Connect + 'static> APIClient<C> {
    pub fn new(configuration: Configuration<C>) -> APIClient<C> {
        let arc = Arc::new(configuration);

        APIClient {
            configuration: arc.clone(),
            approve_api: Box::new(::apis::ApproveApiClient::new(arc.clone())),
        }
    }

    pub fn approve_api(&self) -> &::apis::ApproveApi{
        self.approve_api.as_ref()
    }

}
