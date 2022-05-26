pub mod iot_hub_device;

use async_trait::async_trait;
use orchestration::OrchestrationEngine;

#[derive(Copy, Clone)]
pub struct CloudMessaging {
    on_status_message: fn(&str),
    orchestration: OrchestrationEngine,
}

#[async_trait(?Send)]
pub trait CloudMessagingTrait {
    async fn start(&self);
}
