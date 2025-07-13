use kube::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let _client = Client::try_default().await?;

    anyhow::Ok(())
}
