use async_trait::async_trait;

#[async_trait(?Send)]
pub trait OrchestrationEngineTrait {
    async fn update_manifest(
        &self,
    ) -> std::io::Result<()>;
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct OrchestrationEngine {
    pub on_status_message: fn(&str),
}

#[async_trait(?Send)]
impl OrchestrationEngineTrait for OrchestrationEngine {
    async fn update_manifest(
        &self,
    ) -> std::io::Result<()> {
        (self.on_status_message)("Manifest Updated");
        Ok(())
    }
}

impl OrchestrationEngine {
    pub fn new(status_message_handler: fn(&str)) -> Self {
        Self {
            on_status_message: status_message_handler,
        }
    }
}