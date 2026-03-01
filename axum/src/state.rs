#[derive(Clone, Debug)]
pub struct AppState {
    pub service_name: &'static str,
    pub service_version: &'static str,
}

impl AppState {
    pub fn new(service_name: &'static str, service_version: &'static str) -> Self {
        Self {
            service_name,
            service_version,
        }
    }
}
