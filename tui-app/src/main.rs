use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tui_app::run().await
}

