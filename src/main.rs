use editor::{
    app::{self, AppResult, RunningState},
    buffer::Buffer,
    handler, tui,
};

fn main() -> AppResult<()> {
    let path = std::env::args().nth(1).expect("file name not found");
    let buffer = Buffer::from_file(&path).expect("could not find file");
    tui::install_panic_hook();
    let mut terminal = tui::init()?;
    let mut app = app::App::new(buffer);
    while *app.running_state() == RunningState::Running {
        tui::draw(&mut terminal, &app)?;
        handler::handle_events(&mut app)?; // blocks
    }

    tui::exit()?;
    Ok(())
}
