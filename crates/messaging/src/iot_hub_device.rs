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
            self.process_message().await;

            thread::sleep(Duration::from_millis(1000));
        }
    }
}

#[allow(unused_variables)]
impl CloudMessaging {
    pub fn new(on_status_message: fn(&str)) -> Self {
        
        // Instantiate the orchestration engine
        let orchestration: OrchestrationEngine = OrchestrationEngine {
            on_status_message: |m: &str| {
                println!("Orch Status message from: msg: {}", m);
            },
        };

        let cloud_messaging = CloudMessaging {
            on_status_message,
            orchestration,
        };

        cloud_messaging
    }

    async fn process_message(&self) {
        self.orchestration.update_manifest().await;
    }

    fn set_on_status_message(&mut self, callback: fn(&str)) {
        self.on_status_message = callback;
    }
}
