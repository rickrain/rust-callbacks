use std::{sync::mpsc, thread, alloc::handle_alloc_error, time::Duration};
use async_trait::async_trait;

pub enum ContainerManagementError {
    Error1,
    Error2,
    Unknown,
}

type Result<T> = std::result::Result<T, ContainerManagementError>;

#[async_trait(?Send)]
pub trait ContainerManagementTrait {
    async fn start_container(name: String) -> (mpsc::Receiver<String>, thread::JoinHandle<std::io::Result<()>>);
}

pub struct ContainerManager;

#[async_trait(?Send)]
impl ContainerManagementTrait for ContainerManager {
    async fn start_container(name: String) -> (mpsc::Receiver<String>, thread::JoinHandle<std::io::Result<()>>) {
        let (sender, receiver) = mpsc::channel();
        let container_name = name.clone().to_string();

        let handle = thread::spawn(move || {
            
            // Start the container

            // Monitor the container for changes
            loop {
                sender.send(format!("Container '{}' state changed", container_name));

                thread::sleep(Duration::from_millis(3000));
            }
        });

        (receiver, handle)
    }
}
