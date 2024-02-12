use std::{io::{self, stdout, Write}, env, process::{Command, Stdio}, fs::{self, File}};
use ratatui::{prelude::*, widgets::*};
use tui_textarea::{Input, TextArea, CursorMove, Key};
use crossterm::{
    ExecutableCommand,
    terminal::{disable_raw_mode, LeaveAlternateScreen, enable_raw_mode, EnterAlternateScreen},
};

use crate::app::App;

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

enum TuiMode {
    Password,
    List,
    TextEditor,
    Pager,
}

enum TextMode {
    Add,
    Edit,
}
pub struct TuiApp<'a>{
    mode: TuiMode,
    textarea: TextArea<'a>,
    app: &'a mut App,
    index: usize,
    text_mode: TextMode,
    pager_scroll: u16,
    content: String,
}

enum Operation {
    Nothing,
    Restart,
    Quit,
}

impl <'a>TuiApp <'a>{
    pub fn new(app: &'a mut App) -> Self {
        let textarea = TextArea::default();
        let mut tui_app = TuiApp {
            text_mode: TextMode::Add,
            pager_scroll: 0,
            index: 0,
            mode: TuiMode::List,
            textarea,
            content: String::new(),
            app,
        };

        if tui_app.app.test_passphrase().is_ok() {
            tui_app.set_mode(TuiMode::Password)
        }
        tui_app
    }

    #[inline]
    fn set_mode(&mut self, mode: TuiMode) {
        self.textarea.set_cursor_line_style(Style::default());
        match mode {
            TuiMode::Password => {
                let title = if self.app.empty() {
                    "Initialize directory passphrase"
                } else {
                    "Passphrase"
                };
                self.textarea.set_block(default_block(title));
                self.textarea.set_style(Style::default());
                self.textarea.set_mask_char('\u{2022}')
            },
            TuiMode::TextEditor => {
                self.textarea.clear_mask_char();
                self.textarea.set_block(default_block("Write your new journal"));
            }
            TuiMode::Pager => {
                self.content = self.app.nth_content(self.index);
            }
            TuiMode::List =>  {}
        }
        self.mode = mode;
    }

    #[inline]
    pub fn ui(&mut self, frame:&mut Frame, list_state: &mut ListState) {
        list_state.select(Some(self.index));
        match self.mode {
            TuiMode::Password => {
                self.textarea.set_mask_char('\u{2022}');
                frame.render_widget(self.textarea.widget(),centered_rect(frame.size(), 35, 3));
            }
            TuiMode::TextEditor => {

                self.textarea.clear_mask_char();
                frame.render_widget(self.textarea.widget(),frame.size());
            }
            TuiMode::List => {
                let list = 
                    List::new(self.app.entries())
                    .block(default_block("Journals"))
                    .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

                frame.render_stateful_widget(list, frame.size(), list_state)
            }
            TuiMode::Pager => {
                let paragraph = Paragraph::new(self.content.clone()).scroll((self.pager_scroll,0));
                frame.render_widget(paragraph, frame.size());
            }
        }
    }

    #[inline]
    pub fn increment_index(&mut self) {
        if self.index < self.app.len() - 1 {
            self.index += 1;
        } else {
            self.index = 0
        }
    }

    #[inline]
    pub fn decrement_index(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.app.len() - 1
        }
    }

    fn on_password(&mut self) {
        let passphrase = self.textarea.lines()[0].clone();
        self.app.set_passphrase(passphrase);
        if self.app.test_passphrase().is_ok() {
            self.set_mode(TuiMode::List);
            self.textarea = TextArea::default();
        } else {
            self.textarea.delete_line_by_head();
            self.textarea.delete_line_by_end();
            self.textarea.set_style(Style::default().fg(Color::Red));
            self.textarea.set_block(default_block("Wrong passphrase").fg(Color::Red))
        }
    }

    fn on_new_journal(&mut self) {
        let journal = self.textarea.lines().join("\n");
        self.app.add_journal(journal);
    }

    fn on_edit_journal(&mut self) {
        let journal = self.textarea.lines().join("\n");
        self.app.edit_nth(self.index, journal);
    }

    pub fn input(&mut self) -> io::Result<Operation>{
        match crossterm::event::read()?.into() {
            input => {
            match self.mode {
                TuiMode::Password => {
                    match input {
                        Input {
                            key: Key::Enter,
                            ..
                        } => self.on_password(),
                        Input {
                            key: Key::Esc,
                            ..
                        } => return Ok(Operation::Quit),
                        input => {
                            self.set_mode(TuiMode::Password);
                            self.textarea.input(input);
                        },
                    }
                }
                TuiMode::TextEditor => {
                    match input {
                        Input {
                            key: Key::Esc,
                            ..
                        } => {
                            self.set_mode(TuiMode::List);
                            match self.text_mode {
                                TextMode::Add => self.on_new_journal(),
                                TextMode::Edit => self.on_edit_journal(),
                            }
                            self.textarea = TextArea::default();
                        },
                        input => {
                            self.textarea.input(input);
                        },
                    }
                }
                TuiMode::List => {
                    match input.key {
                        Key::Char('q')=> return Ok(Operation::Quit),
                        Key::Char('a')=> {
                            self.text_mode = TextMode::Add;
                            self.set_mode(TuiMode::TextEditor);
                        },
                        Key::Char('D')=> {
                            self.app.delete_nth(self.index);
                        },
                        Key::Char('e')=> {
                            self.text_mode = TextMode::Edit;
                            self.set_mode(TuiMode::TextEditor);
                            self.textarea.insert_str(self.app.nth_content(self.index));
                        },
                        Key::Enter => {
                            self.set_mode(TuiMode::Pager);
                        }
                        Key::Char('j')=> self.increment_index(),
                        Key::Char('k')=> self.decrement_index(),
                        _ =>{},
                    }
                }
                TuiMode::Pager => {
                    match input.key {
                        Key::Char('j')=> self.pager_scroll+=1,
                        Key::Char('k')=> if self.pager_scroll > 0 {self.pager_scroll-=1},
                        Key::Esc | Key::Char('q')=> {
                                self.content = String::new();
                                self.set_mode(TuiMode::List)
                            },
                        _ => {}
                    }
                }
            }
        }
        }
        Ok(Operation::Nothing)
    }
}

fn centered_rect(r: Rect, percent_x: u16, size_y: u16) -> Rect {
  let popup_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Min(0),
        Constraint::Length(size_y),
        Constraint::Min(0),
    ])
    .split(r);

  Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage((100 - percent_x) / 2),
      Constraint::Percentage(percent_x),
      Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

#[inline]
pub fn run(app: &mut App) -> io::Result<()>{
    startup()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    let mut app = TuiApp::new(app);
    let mut list_state = ListState::default();

    loop {
        terminal.draw(|frame| {
            app.ui(frame, &mut list_state);
        })?;
        match app.input()? {
            Operation::Quit => break,
            Operation::Restart => {
                restart(&mut terminal)?;
            }
            Operation::Nothing => {},
        }
    }
    shutdown()?;
    Ok(())
}
