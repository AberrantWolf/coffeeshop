// use std::{
//     error::Error,
//     io::{stdout, Write},
//     sync::mpsc,
//     thread,
//     time::Duration,
// };

// use tui::backend::CrosstermBackend;
// use tui::layout::{Constraint, Direction, Layout};
// use tui::widgets::{Block, Borders, Widget};
// use tui::Terminal;

// use crossterm::{
//     event::{self, Event as CEvent, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };

// enum UIEvent<I> {
//     Input(I),
//     Tick,
// }

// struct MainInterface {}

// impl MainInterface {
//     fn draw<B: tui::backend::Backend>(&mut self, mut f: &mut tui::Frame<B>) {
//         let chunks = Layout::default()
//             .direction(Direction::Vertical)
//             .margin(1)
//             .constraints(
//                 [
//                     Constraint::Percentage(10),
//                     Constraint::Percentage(80),
//                     Constraint::Percentage(10),
//                 ]
//                 .as_ref(),
//             )
//             .split(f.size());
//         Block::default()
//             .title("Block")
//             .borders(Borders::ALL)
//             .render(&mut f, chunks[0]);
//         Block::default()
//             .title("Block 2")
//             .borders(Borders::ALL)
//             .render(&mut f, chunks[2]);
//     }
// }

// pub fn start_ui() -> Result<(), Box<dyn Error>> {
//     enable_raw_mode()?;

//     let mut stdout = stdout();
//     execute!(stdout, EnterAlternateScreen)?;

//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;
//     terminal.hide_cursor()?;

//     // Setup input handling
//     let (tx, rx) = mpsc::channel();
//     thread::spawn(move || {
//         loop {
//             // poll for tick rate duration, if no events, sent tick event.
//             if event::poll(Duration::from_millis(250)).unwrap() {
//                 if let CEvent::Key(key) = event::read().unwrap() {
//                     tx.send(UIEvent::Input(key)).unwrap();
//                 }
//             }

//             tx.send(UIEvent::Tick).unwrap();
//         }
//     });

//     let mut main_interface = MainInterface {};

//     loop {
//         let result = terminal.draw(|mut f| main_interface.draw(&mut f));
//         match result {
//             Ok(_) => {}
//             Err(_) => break,
//         }

//         match rx.recv()? {
//             UIEvent::Input(event) => match event.code {
//                 KeyCode::Char('q') => {
//                     break;
//                 }
//                 KeyCode::Char(_) => {}
//                 _ => {}
//             },
//             UIEvent::Tick => {}
//         }
//     }

//     // Clean up terminal state
//     disable_raw_mode()?;
//     execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
//     terminal.show_cursor()?;

//     Ok(())
// }
