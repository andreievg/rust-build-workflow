use std::time::Duration;

use tokio::time::sleep;

#[tokio::main]
async fn main() {
    sleep(Duration::from_secs(10)).await;
    println!("2");
}
