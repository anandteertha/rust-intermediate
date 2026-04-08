use crate::app_config::AppConfig;
use std::rc::Rc;

pub struct PaymentsService {
    config: Rc<AppConfig>,
}

impl PaymentsService {
    pub fn new(config: Rc<AppConfig>) -> Self {
        PaymentsService { config }
    }

    pub fn display(self) {
        println!(
            "Service name: Payment Service\nenvironment: {}\n",
            self.config.get_environment()
        )
    }
}
