use orchestration::{OrchestrationEngine,OrchestrationEngineTrait};
use crate::{CloudMessaging,CloudMessagingTrait};
use async_trait::async_trait;
use std::thread;
use std::time::Duration;

#[async_trait(?Send)]
impl CloudMessagingTrait for CloudMessaging {
    async fn start(&self) {
        (self.on_status_message)("Setting up configuration");

        (self.on_status_message)("Starting cloud messaging");

        while true {
            // Do work here
            self.orchestration.update_manifest().await;

            thread::sleep(Duration::from_millis(1000));
        }
    }
}

#[allow(unused_variables)]
impl CloudMessaging {
    pub fn new() -> Self {
        
        // Instantiate the orchestration engine
        let orchestration: OrchestrationEngine = OrchestrationEngine {
            on_status_message: Self::on_status_message,
        };

        let cloud_messaging = CloudMessaging {
            on_status_message: |status: &str| {},
            orchestration,
        };

        cloud_messaging
    }

    fn set_on_status_message(&mut self, callback: fn(&str)) {
        self.on_status_message = callback;
    }

    fn on_status_message(status: &str) {
        println!("Orch Status message: {}", status);
    }
}
