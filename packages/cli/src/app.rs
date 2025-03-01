use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    DefaultTerminal,
};
use std::{
    io::{Read, Result, Write},
    net::TcpStream,
    str ,
};
use tui_input::{Input, InputRequest};

pub struct App {
    input: Input,
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input: Input::default(),
            messages: vec![],
        }
    }
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal, mut stream: TcpStream) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => {
                        let input = self.input.value();

                        let _ = stream.write_all(input.as_bytes());
                        let mut buffer = [0; 1024];
                        let bytes_read = stream.read(&mut buffer)?;
                        let response = str::from_utf8(&buffer[..bytes_read]).unwrap();

                        self.messages.push(format!("Input: {}", input.to_string()));
                        self.messages
                            .push(format!("Response: {}", response.to_string()));

                        let req = InputRequest::DeleteLine;
                        self.input.handle(req);
                    }
                    KeyCode::Char(to_insert) => {
                        let req = InputRequest::InsertChar(to_insert);
                        self.input.handle(req);
                    }
                    KeyCode::Backspace => {
                        let req = InputRequest::DeletePrevChar;
                        self.input.handle(req);
                    }
                    KeyCode::Esc => {
                        break Ok(());
                    }
                    _ => {}
                }
            }
        }
    }

    fn draw(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(f.area());

        let width = chunks[0].width.max(3) - 3;

        let scroll = self.input.visual_scroll(width as usize);
        let input = Paragraph::new(self.input.value())
            .style(Style::default().fg(Color::Yellow))
            .scroll((0, scroll as u16))
            .block(Block::default().borders(Borders::ALL).title("Input"));

        f.render_widget(input, chunks[1]);
        f.set_cursor_position((
            chunks[1].x + ((self.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            chunks[1].y + 1,
        ));

        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = vec![Line::from(Span::raw(format!("{}: {}", i, m)))];
                ListItem::new(content)
            })
            .collect();
        let messages =
            List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
        f.render_widget(messages, chunks[2]);
    }
}
