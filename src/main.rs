use std::sync::mpsc::{self, Sender};

use crate::{app::App, event::Event};
mod app;
mod ui;
mod event;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();

    let (event_tx, event_rx) = mpsc::channel::<Event>();

    let mut app = App::new();
    let app_result = app.run(&mut terminal, event_rx);
    ratatui::restore(); 
    Ok(app_result?)
}

fn handle_input_events(event_tx: Sender<Event>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => event_tx.send(Event::Input(key_event)).unwrap(),
            _ => {}
        }
    }

}
