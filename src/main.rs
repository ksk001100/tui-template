use eyre::Result;
use std::sync::Arc;

// Rename tui_template to package name
use tui_template::app::App;
use tui_template::io::handler::IoAsyncHandler;
use tui_template::io::IoEvent;
use tui_template::start_ui;

#[tokio::main]
async fn main() -> Result<()> {
    let (sync_io_tx, mut sync_io_rx) = tokio::sync::mpsc::channel::<IoEvent>(100);

    let app = Arc::new(tokio::sync::Mutex::new(App::new(sync_io_tx.clone())));
    let app_ui = Arc::clone(&app);

    tokio::spawn(async move {
        let mut handler = IoAsyncHandler::new(app);
        while let Some(io_event) = sync_io_rx.recv().await {
            handler.handle_io_event(io_event).await;
        }
    });

    start_ui(&app_ui).await?;

    Ok(())
}
