pub enum Event {
    Input(crossterm::event::KeyEvent),
    Message(String)
}
