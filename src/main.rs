use demon::cli::cmd;

#[tokio::main]
async fn main() {
    _ = cmd::cli().await
}
