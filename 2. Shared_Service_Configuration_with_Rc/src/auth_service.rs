use crate::app_config::AppConfig;
use std::rc::Rc;

pub struct AuthService {
    pub config: Rc<AppConfig>,
}

impl AuthService {
    pub fn new(config: Rc<AppConfig>) -> Self {
        AuthService { config }
    }

    pub fn display(self) {
        println!(
            "Service name: Auth Service\nenvironment: {}\nretry count: {}\n",
            self.config.get_environment(),
            self.config.get_retry_limit()
        )
    }
}
