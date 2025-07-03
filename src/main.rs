#![allow(unused_must_use)]
use std::io;
use irc::IrcClient;
use std::time::Duration;
use ratatui::{
    DefaultTerminal,

    prelude::{
        Layout,
        Constraint,
    },

    widgets::{
        Paragraph,
    },

    crossterm::event::{
        KeyCode,
        Event,
        read,
        poll,
    },
};

use tui_textarea::TextArea;

fn main() {
    let mut client = IrcClient::new();
    let mut terminal = ratatui::init();

    run(&mut terminal, &mut client);

    ratatui::restore();
}

fn run(terminal: &mut DefaultTerminal, irc: &mut IrcClient) -> io::Result<()> {
    let mut textarea = TextArea::default();

    while irc.active {
        if let Some(rx) = &irc.rx {
            while let Ok(response) = rx.try_recv() {
                irc.lines.push(response);
            }
        }

        
        if irc.lines.len() > 25 {
            irc.lines.drain(0..20);
        }


        terminal.draw(|frame| {
            let vertical_layout = Layout::vertical([
                Constraint::Percentage(10), 
                Constraint::Percentage(80), 
                Constraint::Percentage(10)]
            );

            let [title_area, text_area, input_area] = vertical_layout.areas(frame.area());

            let title = Paragraph::new("IRC client").centered();

            let buf_content = irc.lines.join("\n");
            let text = Paragraph::new(buf_content);

            frame.render_widget(title, title_area);
            frame.render_widget(text, text_area);
            frame.render_widget(&textarea, input_area);
        })?;


        if poll(Duration::from_millis(100))? {
            if let Event::Key(key) = read()? {
                if key.code == KeyCode::Esc {
                    irc.active = false;
                }

                if key.code == KeyCode::Enter {
                    let command = textarea.clone().into_lines().remove(0);
                    textarea = TextArea::new(Vec::<String>::new());

                    match irc.parse_command(&command) {
                        Ok(argv) => match irc.lexer(&argv) {
                            Ok(tokenized_cmd) => {
                                irc.execute_command(tokenized_cmd);
                            }

                            Err(_) => {
                                irc.lines.push("Error while lexing the command, check the number of arguments".to_string());
                            }
                        },

                        Err(e) => {
                            irc.lines.push(e);
                        }
                    }
                }
                else {
                    textarea.input(tui_textarea::Input::from(key));
                }
            }
        }

    }
    Ok(())
}