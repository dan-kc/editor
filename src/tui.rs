use crate::{app::App, logger::Logger, ui};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{io, panic};

pub fn init() -> io::Result<Terminal<impl Backend>> {
    terminal::enable_raw_mode()?;
    crossterm::execute!(io::stdout(), EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    Ok(terminal)
}

pub fn exit() -> io::Result<()> {
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

pub fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        exit().expect(
            "custom panic hook failed i'm so sorry i fucked up your terminal",
        );
        original_hook(panic_info);
    }));
}

pub fn draw(
    terminal: &mut Terminal<impl Backend>,
    app: &App,
    logger: &Logger,
) -> io::Result<()> {
    terminal.draw(|frame| ui::render(app, logger, frame))?;
    Ok(())
}
