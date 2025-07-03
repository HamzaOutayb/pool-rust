use chrono::Duration;
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
	Top,
	Bottom,
	Center,
}

#[derive(Debug, Eq, PartialEq)]
 pub struct Notification {
	pub size: u32,
	pub color: (u8, u8, u8),
	pub position: Position,
	pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
	Remainder(&'a str),
	Registration(Duration),
	Appointment(&'a str),
	Holiday,
}

use std::fmt;

impl fmt::Display for Notification {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (r, g, b) = self.color;
        let colored_content = self.content.truecolor(r, g, b);
        write!(f, "({}, {}, {})", match self.position {
            Position::Top => "Top",
            Position::Bottom => "Bottom",
            Position::Center => "Center",
        }, self.size, colored_content)
    }
}

use Event::*;

impl Event<'_> {
    pub fn notify(&self) -> Notification {
        match self {
            Event::Remainder(text) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: text.to_string(),
            },
            Event::Registration(time) => {
                let total_seconds = time.num_seconds();

                let hours = total_seconds / 3600;
                let minutes = (total_seconds % 3600) / 60;
                let seconds = total_seconds % 60;

                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content: format!(
                        "You have {}H:{}M:{}S left before the registration ends",
                        hours, minutes, seconds
                    ),
                }
            }
            Event::Appointment(text) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: String::from(*text),
            },
            Event::Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: String::from("Enjoy your holiday"),
            },
        }
    }
}