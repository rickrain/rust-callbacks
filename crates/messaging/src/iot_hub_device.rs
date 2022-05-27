use orchestration::{OrchestrationEngine,OrchestrationEngineTrait,OrchestrationError};
use crate::{CloudMessaging,CloudMessagingTrait,StatusCallback,ErrorCallback};
use async_trait::async_trait;
use std::thread;
use std::time::Duration;

#[async_trait(?Send)]
impl CloudMessagingTrait for CloudMessaging {
    async fn start(&self) {
        (self.status_callback)("Setting up configuration");

        (self.status_callback)("Starting cloud messaging");

        loop {
            let update_manifest_result = self.orchestration_engine.update_manifest().await;
            match update_manifest_result {
                Ok(s) => (self.status_callback)(&s),
                Err(error) => {
                    match error {
                        OrchestrationError::Crashed(s) => panic!("Orchestration engine crashed. Error message: {}", s),
                        OrchestrationError::Timeout(x) => panic!("Orchestration engine timed out after '{}' seconds.", x),
                        OrchestrationError::Unknown => panic!("Orchestration engine encountered an unknown error."), 
                    }
            }
        }
        thread::sleep(Duration::from_millis(2000));
    }
    }}


#[allow(unused_variables)]
impl CloudMessaging {
    pub fn new() -> Self {
        
        // Instantiate the orchestration engine
        let mut orchestration_engine = OrchestrationEngine::new();

        // orchestration_engine.add_status_handler(Box::new(Self::on_status));
        // orchestration_engine.add_error_handler(Box::new(Self::on_error));

        let cloud_messaging = CloudMessaging {
            orchestration_engine,
            status_callback: Box::new(|_| {}),
            error_callback: Box::new(|_| {}),
        };

        cloud_messaging
    }

    pub fn add_status_handler(&mut self, callback: StatusCallback) {
        self.status_callback = callback;
    }

    pub fn add_error_handler(&mut self, callback: ErrorCallback) {
        self.error_callback = callback;
    }

    // fn on_status(status: &str ) {
    //     println!("Orchestration status: {}", status);
    // }

    // fn on_error(error: OrchestrationError) {
    //     match error {
    //         OrchestrationError::Crashed(s) => println!("Orchestration engine crashed. Error message: {}", s),
    //         OrchestrationError::Timeout(x) => println!("Orchestration engine timed out after '{}' seconds.", x),
    //         OrchestrationError::Unknown => println!("Orchestration engine encountered an unknown error."), 
    //     }
    // }
}
