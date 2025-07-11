pub mod app;
pub mod brand;
pub mod components;
pub use app::App;

pub async fn run() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}
