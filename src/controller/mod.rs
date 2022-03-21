pub mod view;

use std::io;

use crate::controller::view::View;
use crate::events::Global;

use crossterm;
use crossterm::event::Event as ctEvent;
use crossterm::event::KeyCode;

#[derive(Default)]
pub struct Controller {
    pub views: Vec<View>,
}

impl Controller {
    pub fn new(files: &Vec<String>) -> io::Result<Self> {
        let mut views = vec![];

        for file_path in files {
            let view = View::new(file_path.to_string())?;
            views.push(view);
        }

        Ok(Self { views })
    }

    pub fn handle_event(&mut self, event: ctEvent) -> Result<Global, String> {
        match event {
            ctEvent::Key(key) => {
                // https://docs.rs/crossterm/0.17.7/crossterm/event/enum.KeyCode.html
                match key.code {
                    KeyCode::Char('q') => return Ok(Global::Exit),
                    // KeyCode::Char(value) => self.buffer.content[self.line].push(value),
                    _ => {
                        //
                    }
                };
            }
            ctEvent::Mouse(_event) => {}
            ctEvent::Resize(_width, _height) => {}
        }

        Ok(Global::Noop)
    }
}
