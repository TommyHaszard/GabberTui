use std::sync::mpsc::{Receiver};

use ratatui::{DefaultTerminal};

use crate::event::Event;

pub enum CurrentScreen {
    Main,
    SetupUser,
    InitialiseRelationship,
    Exiting,
}

pub enum MainMode {
    Normal,
    Input,
}

pub enum SetupUserInput {
    Name,
}

pub enum InitialiseRelationshipMode {
    Searching,
    Listening,
}


pub enum InitialiseRelationshipError {}

pub struct App {
    pub current_screen: CurrentScreen,
    pub main_mode: MainMode,
    pub setup_user_input: Option<SetupUserInput>,
    pub init_relationship_mode: Option<InitialiseRelationshipMode>,
    pub input: String,
    pub chat_users: Vec<String>,
    pub exiting: bool,
    pub username: String,
    pub events: Option<Receiver<Event>>,
    pub initialise_relationship_error: Option<InitialiseRelationshipError>,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            main_mode: MainMode::Normal,
            setup_user_input: None,
            init_relationship_mode: None,
            input: String::new(),
            chat_users: Vec::new(),
            exiting: false,
            username: String::new(),
            events: None,
            initialise_relationship_error: None,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal, event_rx: Receiver<Event>) -> std::io::Result<()> {
        Ok(())        
    }

}
