use anyhow::Result;
use crossterm::{
    event::{Event, KeyCode, KeyEvent},
    execute,
    style::{Print, ResetColor, SetBackgroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{stdout, Stdout, Write};
use tui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Terminal,
};

struct BebraCommand<'a> {
    name: String,
    func: Box<dyn Fn() + 'a>,
}
impl<'a> BebraCommand<'a> {
    fn new(name: String, func: impl Fn() + 'a) -> Self {
        BebraCommand {
            name,
            func: Box::new(func),
        }
    }
    fn execute(&self) {
        (self.func)()
    }
}

pub fn create_terminal() -> Result<()> {
    enable_raw_mode().unwrap();
    execute!(stdout(), Clear(ClearType::All)).unwrap();

    ui()?;

    disable_raw_mode().unwrap();
    execute!(stdout(), Clear(ClearType::All)).unwrap();

    Ok(())
}

fn ui() -> Result<()> {
    let commands = vec![
        BebraCommand::new(
            "Никита Серов нюхает бебру".to_string(),
            &|| println!("{}", "Никита Серов нюхает бебру".to_string()),
        ),
        BebraCommand::new("Елисей нюхает бебру".to_string(), || {
            println!("{}", "Елисей нюхает бебру".to_string())
        }),
        BebraCommand::new("Карнаж нюхает бебру".to_string(), || {
            println!("{}", "Карнаж нюхает бебру".to_string())
        }),
        BebraCommand::new("Шамиль нюхает бебру".to_string(), || {
            println!("{}", "Шамиль нюхает бебру".to_string())
        }),
    ];
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut selected_index = 0;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let items: Vec<ListItem> = commands
                .iter()
                .enumerate()
                .map(|(i, command)| {
                    if i == selected_index {
                        ListItem::new(format!("{}. {}", i + 1, command.name)).style(
                            Style::default()
                                .fg(tui::style::Color::Green)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else {
                        ListItem::new(format!("{}. {}", i + 1, command.name))
                    }
                })
                .collect();

            let list = List::new(items).block(
                Block::default()
                    .border_style(Style {
                        fg: Some(Color::LightYellow),
                        bg: Some(Color::Black),
                        add_modifier: Modifier::ITALIC,
                        sub_modifier: Modifier::BOLD,
                    })
                    .border_type(BorderType::Thick)
                    .borders(Borders::ALL)
                    .title("Anibebra"),
            );

            f.render_widget(list, size);
        })?;

        if let Ok(Event::Key(KeyEvent { code, .. })) = crossterm::event::read() {
            match code {
                KeyCode::Char('q') => break,
                KeyCode::Up => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_index < commands.len() - 1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Enter => {
                    commands[selected_index].execute();
                }
                _ => {}
            }
        }
    }

    Ok(())
}
