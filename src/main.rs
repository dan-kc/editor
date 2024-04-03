use editor::app::{App, AppResult};
use editor::event::{Event, EventHandler};
use editor::handler::handle_key_events;
use editor::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

#[tokio::main]
async fn main() -> AppResult<()> {
    let path = std::env::args().nth(1).expect("no path given");
    let buffer = editor::buffer::Buffer::from_file(path).expect("could not find file");

    let mut app = App::new(buffer);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;
        match tui.events.next().await? {
            // Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            _ => {}
        }
    }

    app.logger.write_to_file()?;
    tui.exit()?;
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use std::time::Duration;
//
//     use super::*;
//     use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
//     use editor::event::Event;
//     use ratatui::assert_buffer_eq;
//     use tokio::sync::mpsc::{self};
//     use tokio::sync::oneshot;
//
//     #[tokio::test]
//     async fn test_key_press() {
//         let (event_sender, event_receiver) = mpsc::unbounded_channel();
//         let (shutdown_sender, shutdown_reciever) = oneshot::channel();
//         let buffer: editor::buffer::Buffer = Default::default();
//         let backend = CrosstermBackend::new(io::stderr());
//         let terminal = Terminal::new(backend).unwrap();
//
//         // Spawn the main function in a separate task
//         let main_task = tokio::spawn(async move {
//             let mut app = App::new(buffer);
//
//             let events = EventHandler::new(250);
//             let mut tui = Tui::new(terminal, events);
//
//             tui.init().unwrap();
//
//             while app.running {
//                 tui.draw(&mut app).unwrap();
//                 match tui.events.next().await.unwrap() {
//                     Event::Tick => app.tick(),
//                     Event::Key(key_event) => handle_key_events(key_event, &mut app).unwrap(),
//                     _ => {}
//                 }
//             }
//
//             tui.exit().unwrap();
//             shutdown_sender
//                 .send(())
//                 .expect("Failed to send shutdown signal");
//         });
//
//         // Simulate pressing a key by sending a key event
//         event_sender
//             .send(Event::Key(KeyEvent::new(
//                 KeyCode::Char('a'),
//                 KeyModifiers::NONE,
//             )))
//             .expect("Failed to send key event");
//
//         // Wait for a short duration to allow the main function to process the event
//         tokio::time::sleep(Duration::from_millis(100)).await;
//
//         // Receive the state of the application after processing the key event
//
//         // Assert that the application state has been updated as expected
//         let expected =
//             ratatui::buffer::Buffer::with_lines(vec!["┏━━━━━━━━━━━━━ Editor  ━━━━━━━━━━━━━┓"]);
//
//         assert_eq!(*terminal.current_buffer_mut(), expected);
//         //
//         // // Shutdown the main function
//         shutdown_reciever
//             .await
//             .expect("Failed to shutdown main function");
//     }
// }
