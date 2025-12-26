use anyhow::Result;
use email_newsletter::run;

#[tokio::main]
async fn main() -> Result<()> {
    run()?.await?;
    Ok(())
}
