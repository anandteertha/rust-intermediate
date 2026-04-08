use std::rc::Rc;

use crate::{
    api_gateway_service::ApiGateService, app_config::AppConfig, auth_service::AuthService,
    payments_service::PaymentsService,
};
mod api_gateway_service;
mod app_config;
mod auth_service;
mod payments_service;

fn main() {
    let config: Rc<AppConfig> =
        AppConfig::new("environment".to_owned(), "database_url".to_owned()).into();

    // create each service.
    let api_gateway_service = ApiGateService::new(Rc::clone(&config));
    let payments_service = PaymentsService::new(Rc::clone(&config));
    let auth_service = AuthService::new(Rc::clone(&config));

    api_gateway_service.display();
    payments_service.display();
    auth_service.display();
}
