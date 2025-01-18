pub enum CurrentScreen {
    StartMenu,
    InGame,
}

pub enum CurrentSelection {
    NewGame,
    Exit,
}

pub struct App {
    pub current_screen: CurrentScreen, 
    pub current_selection: Option<CurrentSelection>, 
}