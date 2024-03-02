#[tokio::main]
async fn main() {
    if let Err(err) = rust_and_dungeons::start().await {
        eprintln!("{err}");
    }
}
