use messaging::{CloudMessaging,CloudMessagingTrait};

#[tokio::main]
async fn main() {
    println!("Starting microEdge!");

    let mut cloud_messaging = CloudMessaging::new();
    cloud_messaging.add_status_handler(Box::new(on_status));
    cloud_messaging.add_error_handler(Box::new(on_error));
    
    cloud_messaging.start().await;

    //println!("Stopping microEdge...");
}

fn on_status(status_msg: &str) {
    println!("CloudMessaging status: {}", status_msg);
}

fn on_error(error_msg: &str) {
    println!("CloudMessaging error: {}", error_msg);
}
