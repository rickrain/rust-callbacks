use async_trait::async_trait;

pub enum OrchestrationError {
    Crashed(String),
    Timeout(u32),
    Unknown,
}

type Result<T> = std::result::Result<T, OrchestrationError>;

type StatusCallback = Box<dyn Fn(&str)>;
type ErrorCallback = Box<dyn Fn(OrchestrationError)>;

#[async_trait(?Send)]
pub trait OrchestrationEngineTrait {
    async fn update_manifest(&self) -> Result<String>;
}

#[allow(dead_code)]
//#[derive(Copy, Clone)]
pub struct OrchestrationEngine {
    status_callback: StatusCallback,
    error_callback: ErrorCallback,
}

#[async_trait(?Send)]
impl OrchestrationEngineTrait for OrchestrationEngine {
    async fn update_manifest(&self) -> Result<String> {
        // // Update manifest here
        // (self.status_callback)("Manifest Updated");
        
        // // Simulate an error - Timeout after 300 seconds
        // (self.error_callback)(OrchestrationError::Timeout(300));

        // Figure out what's running

        // Compare manifest differences

        // Call into container to make the changes

        Ok("Manifest updated!".to_string())

        // Simulate an error - Timeout after 300 seconds
        //Err(OrchestrationError::Timeout(30))
    }
}

impl OrchestrationEngine {
    pub fn new() -> Self {
        Self {
            status_callback: Box::new(|_| {}), 
            error_callback: Box::new(|_| {}), 
        }
    }

    pub fn add_status_handler(&mut self, callback: StatusCallback) {
        self.status_callback = callback;
    }

    pub fn add_error_handler(&mut self, callback: ErrorCallback) {
        self.error_callback = callback;
    }

}