pub mod iot_hub_device;

use async_trait::async_trait;
use orchestration::{OrchestrationEngine,OrchestrationError};

type StatusCallback = Box<dyn Fn(&str)>;
//type ErrorCallback = Box<dyn Fn(OrchestrationError)>;
type ErrorCallback = Box<dyn Fn(&str)>;

//#[derive(Copy, Clone)]
pub struct CloudMessaging {
    orchestration_engine: OrchestrationEngine,
    status_callback: StatusCallback,
    error_callback: ErrorCallback,
}

#[async_trait(?Send)]
pub trait CloudMessagingTrait {
    async fn start(&self);
}
