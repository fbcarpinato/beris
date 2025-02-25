use ratatui::{style::Style, widgets::Widget};

pub struct CommandInputWidget {
    input: String
}

impl CommandInputWidget {
    pub fn new() -> Self {
        Self {
            input: String::from("test")
        }
    }
}

impl Widget for CommandInputWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        buf.set_string(area.left(), area.top(), &self.input, Style::default());
    }
}
