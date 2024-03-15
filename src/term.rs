use std::io::Write;

use crossterm::event::{Event, read, KeyCode};
use crossterm::cursor;
use crossterm::terminal;
use crossterm::style::{Print, ResetColor};


pub fn handle_events() -> std::io::Result<()> {
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;

    let term_size = terminal::size()?;
    queue!(stdout, terminal::EnterAlternateScreen, cursor::MoveToRow(term_size.1), cursor::MoveToColumn(0))?;

    loop {
        stdout.flush()?;

        // reset the cursor
        queue!(
            stdout,
            ResetColor,
            cursor::Show,
        )?;

        // `read()` blocks until an `Event` is available
        match read()? {
            Event::FocusGained => println!("FocusGained"),
            Event::FocusLost => println!("FocusLost"),
            Event::Key(event) => {
                match event.code {
                    KeyCode::Char(c) => {
                        execute!(stdout, Print(c))?;
                    },
                    KeyCode::Backspace => queue!(stdout, cursor::MoveLeft(1), Print(" "), cursor::MoveLeft(1))?,
                    KeyCode::Enter => queue!(stdout, Print("\n"), cursor::MoveToColumn(0), cursor::MoveToNextLine(1))?,
                    KeyCode::Esc => break,
                    _ => execute!(stdout, Print("unknown"))?,
                }
            },
            Event::Mouse(event) => println!("{:?}", event),
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
            _ => println!("unrecognized event")
        }
    }

    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
