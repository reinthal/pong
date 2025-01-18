use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Rect, Alignment},
    style::{Stylize, Color, Style},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use std::io;

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
    pub exit: bool
}

impl App {

    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::StartMenu,
            current_selection: Some(CurrentSelection::NewGame),
            exit: false
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => self.handle_direction_event(),
            KeyCode::Down => self.handle_direction_event(),
            _ => {}
        }
    }
    fn handle_direction_event(&mut self) {
        match self.current_selection {
            Some(CurrentSelection::NewGame) => self.current_selection = Some(CurrentSelection::Exit),
            Some(CurrentSelection::Exit) => self.current_selection = Some(CurrentSelection::NewGame),
            None => ()
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" PONG ".bold());
        let instructions = Line::from(vec![
            " Move: ".into(),
            "<Up>/<Down>".blue().bold(),
            " Choose: ".into(),
            "<Enter>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        let mut lines = vec![];

        lines.push(Line::from(vec![
            Span::raw(if matches!(self.current_selection, Some(CurrentSelection::NewGame)) {
                "◉ "
            } else {
                "  "
            }),
            Span::styled("New Game", Style::default().fg(Color::Yellow)),
        ]));

        // Dynamically style "Exit"
        lines.push(Line::from(vec![
            Span::raw(if matches!(self.current_selection, Some(CurrentSelection::Exit)) {
                "◉ "
            } else {
                "  "
            }),
            Span::styled("Exit", Style::default().fg(Color::Yellow)),
        ]));

        let main_menu = Text::from(lines);

        Paragraph::new(main_menu)
            .alignment(Alignment::Center)
            .centered()
            .block(block)
            .render(area, buf);
    }
}