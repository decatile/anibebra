mod ui;

use anyhow::Result;
use crossterm::terminal::enable_raw_mode;
use ui::terminal::create_terminal;

#[tokio::main]
async fn main() -> Result<()> {
    create_terminal();

    Ok(())
}
