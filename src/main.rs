mod controller;
mod events;
use crossterm;
use events::Global;
use tui::{
    self,
    layout::{Constraint, Direction, Layout},
    text::Spans,
    widgets::{Paragraph, Wrap},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().peekable();
    // skip first, we dont care about it
    args.next();

    let mut files = vec![];
    while let Some(arg) = args.next() {
        files.push(arg.to_string())
    }

    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();

    let mut terminal = setup_terminal(&mut stdout)?;

    let app = setup_app(&mut terminal, &files);

    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    if let Err(err) = app {
        println!("{:?}", err)
    }

    Ok(())
}

/// Sets up the tui terminal with crossterm
///
/// * `stdout`:
fn setup_terminal(
    stdout: &mut std::io::Stdout,
) -> Result<
    tui::Terminal<tui::backend::CrosstermBackend<&mut std::io::Stdout>>,
    Box<dyn std::error::Error>,
> {
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture,
    )?;

    let backend = tui::backend::CrosstermBackend::new(stdout);
    let terminal = tui::Terminal::new(backend)?;

    Ok(terminal)
}

fn setup_app<B: tui::backend::Backend>(
    terminal: &mut tui::Terminal<B>,
    files: &Vec<String>,
) -> std::io::Result<()> {
    let mut c = controller::Controller::new(&files)?;

    loop {
        terminal.draw(|frame: &mut tui::Frame<B>| {
            let size = frame.size();

            /*
            let block = tui::widgets::Block::default()
                .borders(tui::widgets::Borders::ALL)
                .title("Title")
                .title_alignment(tui::layout::Alignment::Center)
                .border_type(tui::widgets::BorderType::Rounded);

            frame.render_widget(block, size);
            */

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Percentage(100)])
                .split(size);

            match c.views.len() {
                0 => {
                    let paragraph = Paragraph::new("No Buffer Selected");
                    // .style(Style::default().bg(Color::White).fg(Color::White));

                    frame.render_widget(paragraph, chunks[0]);
                }
                _ => {
                    for view in c.views.iter() {
                        let mut text = vec![];

                        for line in view.rope.lines() {
                            text.push(Spans::from(line.to_string()));
                        }

                        let paragraph = Paragraph::new(text)
                            // .style(Style::default().bg(Color::White).fg(Color::Black))
                            // .block(...)
                            // .alignment(Alignment::Right)
                            .wrap(Wrap { trim: true });

                        frame.render_widget(paragraph, chunks[0]);
                    }
                }
            }
        })?;

        // prevent blocking
        //  (crossterm::event::read()) is a blocking operation, so we need to poll to prevent it
        //  from blocking
        // if crossterm::event::poll(Duration::from_millis(100))? { ... }

        match c.handle_event(crossterm::event::read()?) {
            Ok(action) => match action {
                Global::Exit => {
                    return Ok(());
                }
                _ => {}
            },
            Err(reason) => {
                panic!("Panic with c.handle_event: {}", reason);
            }
        }
    }
}
