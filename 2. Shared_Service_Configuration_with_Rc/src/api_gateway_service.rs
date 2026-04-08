use crate::app_config::AppConfig;
use std::rc::Rc;

pub struct ApiGateService {
    config: Rc<AppConfig>,
}

impl ApiGateService {
    pub fn new(config: Rc<AppConfig>) -> Self {
        ApiGateService { config }
    }
    pub fn display(self) {
        println!(
            "Service name: Api gateway Service\nenvironment: {}\ndatabase name: {}\n",
            self.config.get_environment(),
            self.config.get_database_url()
        )
    }
}
