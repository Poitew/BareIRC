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
        Borders,
        Block,
        Wrap,
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
    client.send_message("\n".to_string());
    let mut terminal = ratatui::init();

    run(&mut terminal, &mut client);

    ratatui::restore();
}

fn run(terminal: &mut DefaultTerminal, irc: &mut IrcClient) -> io::Result<()> {
    let mut textarea = TextArea::default();

    while irc.active {
        irc.auto_scroll = true;

        let messages: Vec<_> = if let Some(rx) = &irc.rx {
            std::iter::from_fn(|| rx.try_recv().ok()).collect()
        } 
        else {
            vec![]
        };

        for msg in messages {
            irc.send_message(msg);
        }

        if irc.lines.len() > 250 {
            irc.lines.drain(0..50);
        }


        terminal.draw(|frame| {
            let vertical_layout = Layout::vertical([
                Constraint::Percentage(15),
                Constraint::Percentage(85),
            ]);

            let horizontal_layout = Layout::horizontal([
                Constraint::Percentage(15), 
                Constraint::Percentage(85),
            ]).spacing(2);

            let vertical_layout_main = Layout::vertical([
                Constraint::Percentage(85),
                Constraint::Percentage(15),
            ]);

            let [title_area, main_area] = vertical_layout.areas(frame.area());
            let [channel_area, text_area] = horizontal_layout.areas(main_area);
            let [log_area, input_area] = vertical_layout_main.areas(text_area);



            let title = Paragraph::new(r#"
 ___               ___ ___  ___ 
| _ ) __ _ _ _ ___|_ _| _ \/ __|
| _ \/ _` | '_/ -_)| ||   / (__ 
|___/\__,_|_| \___|___|_|_\\___|
            "#)
            .centered()
            .block(Block::default()
                .borders(Borders::BOTTOM)
            );


            let buf_content = irc.lines.join("");
            
            let text = Paragraph::new(buf_content)
            .wrap(Wrap { trim: false })
            .scroll((irc.scroll_offset, 0));

            let buf_content = irc.channels.join("\n\n");
            let channels = Paragraph::new(buf_content).
                block(Block::default()
                    .borders(Borders::RIGHT)
                );



            frame.render_widget(title, title_area);
            frame.render_widget(channels, channel_area);
            frame.render_widget(text, log_area);
            frame.render_widget(&textarea, input_area);
        })?;


        if poll(Duration::from_millis(100))? {
            if let Event::Key(key) = read()? {
                if key.code == KeyCode::Esc {
                    irc.active = false;
                }

                if key.code == KeyCode::Up {
                    irc.auto_scroll = false;
                    irc.scroll_offset = irc.scroll_offset.saturating_sub(1);
                }

                if key.code == KeyCode::Down {
                    irc.scroll_offset += 1;
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
                                irc.send_message("Error while lexing the command, check the number of arguments\n".to_string());
                            }
                        },

                        Err(e) => {
                            irc.send_message(e);
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