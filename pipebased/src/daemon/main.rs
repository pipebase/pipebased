mod daemon;

use tracing::instrument;

#[tokio::main]
#[instrument]
async fn main() {
    println!("Hello, world!");
}
