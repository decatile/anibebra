use anyhow::Result;
use crossterm::{
    cursor,
    event::{
        poll, read, DisableFocusChange, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
        KeyModifiers,
    },
    style::{self, Color, Print, PrintStyledContent, SetForegroundColor, StyledContent, Stylize},
    terminal::{self, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use rand::Rng;
use std::{
    io::{stdout, Write},
    time::Duration,
};

fn main() -> Result<()> {
    let mut stdout = stdout();

    enable_raw_mode().unwrap();

    stdout
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(SetForegroundColor(Color::Blue))?
        .execute(EnableMouseCapture)?
        .execute(EnterAlternateScreen)?;

    for y in 0..40 {
        for x in 0..150 {
            if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
                let mut rng = rand::thread_rng();
                let random = rng.gen_range(0..5);

                let mut styled_content = match random {
                    0 => "██".green(),
                    1 => "██".white(),
                    2 => "██".red(),
                    3 => "██".blue(),
                    4 => "██".yellow(),
                    _ => "██".magenta(),
                };
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(PrintStyledContent(styled_content))?;
            }
        }
    }

    print_events()?;

    stdout.execute(LeaveAlternateScreen)?;
    stdout.flush()?;
    Ok(())
}

fn print_events() -> crossterm::Result<()> {
    loop {
        match read()? {
            Event::Key(event) => match (event.code, event.modifiers) {
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    println!("GG");
                    break;
                }
                _ => println!("LET'S GO"),
            },

            _ => println!("bebra"),
        }
    }
    Ok(())
}
