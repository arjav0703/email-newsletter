use anyhow::Result;
use email_newsletter::run;

#[tokio::main]
async fn main() -> Result<()> {
    run("127.0.0.1:8080")?.await?;
    Ok(())
}
