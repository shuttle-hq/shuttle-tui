// ANCHOR: all
use clap::Parser;
use color_eyre::eyre::Result;
use shuttle_tui::{
    app::App,
    args::Args,
    shuttle::Shuttle,
    utils::{initialize_logging, initialize_panic_handler},
};

async fn tokio_main() -> Result<()> {
    initialize_logging()?;

    initialize_panic_handler()?;

    let args = Args::parse();
    let shuttle = Shuttle;
    let mut app = App::new(shuttle, &args)?;
    app.run().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = tokio_main().await {
        eprintln!("{} error: Something went wrong", env!("CARGO_PKG_NAME"));
        Err(e)
    } else {
        Ok(())
    }
}
// ANCHOR_END: all
