use std::{thread::{self, JoinHandle}, sync::mpsc::Receiver};
use async_trait::async_trait;
use container_mgmt::{ContainerManager, ContainerManagementTrait};

pub enum OrchestrationError {
    Crashed(String),
    Timeout(u32),
    Unknown,
}

type Result<T> = std::result::Result<T, OrchestrationError>;

type StatusCallback = Box<dyn Fn(&str) + Send + Sync + 'static>;
type ErrorCallback = Box<dyn Fn(OrchestrationError) + Sync>;

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
        // Figure out what's running

        // Compare manifest differences

        // Start a container
        let container_name = "c1".to_string();
        let container = ContainerManager::start_container(container_name).await;

        let handle = thread::spawn(move || {
            let receiver = container.0;

            while let Ok(message) = receiver.recv() {
                //(self.status_callback)(&message);
            }
        });

        Ok("Manifest updated!".to_string())

        // Simulate an error - Timeout after 300 seconds
        //Err(OrchestrationError::Timeout(30))
    }
}

impl OrchestrationEngine {
    pub fn new() -> Self {
        OrchestrationEngine {
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