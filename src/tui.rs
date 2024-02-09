use std::{io::{self, stdout}, rc::Rc};
use tui_textarea::{Input, TextArea, CursorMove};
use ratatui::{prelude::*, widgets::*};
use crossterm::{
    ExecutableCommand,
    terminal::{disable_raw_mode, LeaveAlternateScreen, enable_raw_mode, EnterAlternateScreen},
    event::{self, Event::Key, KeyCode::Char, KeyCode},
};

pub fn default_block<'a, T>(title: T) -> Block<'a> 
where
    T: Into<Line<'a>>,
{
    Block::default().title(title).borders(Borders::ALL)
}

pub enum DirectoryWidget<'a> {
    List(ratatui::widgets::List<'a>),
    Paragraph(ratatui::widgets::Paragraph<'a>),
}

/// Shutdown TUI app (undo everything did in startup, and show cursor)
pub fn shutdown() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(crossterm::cursor::Show)?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

/// Prepare terminal for TUI applicaton by enabling rowmode and entering alternate screen.
pub fn startup() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Ok(())
}

/// Restart terminal
#[inline]
pub fn restart(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    terminal.clear()?;
    startup()?;
    Ok(())
}

// pub fn create_directory_widget(directory:Directory) ->  DirectoryWidget {

// }
