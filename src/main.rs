use messaging::{CloudMessaging,CloudMessagingTrait};

#[tokio::main]
async fn main() {
    println!("Starting microEdge!");

    let cloud_messaging = CloudMessaging::new(on_status_message);

    cloud_messaging.start().await;

    println!("Stopping microEdge...");
}

fn on_status_message(status: &str) {
    println!("Connected to messaging layer: {}", status);
}
