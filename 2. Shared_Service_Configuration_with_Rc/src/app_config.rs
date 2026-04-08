pub struct AppConfig {
    environment: String,
    database_url: String,
    retry_limit: usize,
}

impl AppConfig {
    pub fn new(environment: String, database_url: String) -> Self {
        AppConfig {
            environment,
            database_url,
            retry_limit: 3,
        }
    }

    pub fn get_environment(&self) -> &str {
        &self.environment
    }
    pub fn get_database_url(&self) -> &str {
        &self.database_url
    }
    pub fn get_retry_limit(&self) -> usize {
        self.retry_limit
    }
}
