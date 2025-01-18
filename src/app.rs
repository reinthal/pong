use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use std::io;

#[derive(PartialEq)]
pub enum CurrentScreen {
    StartMenu,
    InGame,
}

pub enum Direction {
    Up,
    Down,
    Na,
}

pub struct Player {
    pub y: u32,
    pub lifes: u32,
    pub direction: Direction,
}

impl Player {
    pub fn new() -> Player {
        Player {
            y: 50,
            lifes: 3,
            direction: Direction::Na,
        }
    }
}

pub enum CurrentSelection {
    NewGame,
    Exit,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub current_selection: Option<CurrentSelection>,
    pub exit: bool,
    pub p1: Player,
    pub p2: Player,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::StartMenu,
            current_selection: Some(CurrentSelection::NewGame),
            exit: false,
            p1: Player::new(),
            p2: Player::new(),
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
            Event::Key(key_event) if key_event.kind == KeyEventKind::Release => {
                self.handle_key_release_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_release_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            // p1 stop
            KeyCode::Char('w') => self.p1.direction = Direction::Na,
            KeyCode::Char('s') => self.p1.direction = Direction::Na,
            // p2 stop
            KeyCode::Up => self.p2.direction = Direction::Na,
            KeyCode::Down => self.p2.direction = Direction::Na,
            _ => {}
        }
    }
    fn handle_q_event(&mut self) {
        if self.current_screen == CurrentScreen::InGame {
            self.current_screen = CurrentScreen::StartMenu;
        } else {
            self.exit();
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.handle_q_event(),
            KeyCode::Enter => self.handle_selection_event(),
            KeyCode::Up => self.handle_direction_press_event(key_event),
            KeyCode::Down => self.handle_direction_press_event(key_event),
            KeyCode::Char('w') => self.handle_direction_press_event(key_event),
            KeyCode::Char('s') => self.handle_direction_press_event(key_event),
            _ => {}
        }
    }
    fn handle_selection_event(&mut self) {
        match self.current_selection {
            Some(CurrentSelection::NewGame) => self.current_screen = CurrentScreen::InGame,
            Some(CurrentSelection::Exit) => {
                self.current_selection = None;
                self.exit();
            }
            _ => {}
        }
    }
    fn handle_direction_press_event(&mut self, key_event: KeyEvent) {
        match self.current_screen {
            CurrentScreen::InGame => match key_event.code {
                // Move p1
                KeyCode::Char('w') => self.p1.direction = Direction::Up,
                KeyCode::Char('s') => self.p1.direction = Direction::Down,
                // Move p2
                KeyCode::Up => self.p2.direction = Direction::Up,
                KeyCode::Down => self.p2.direction = Direction::Down,
                _ => {}
            },
            CurrentScreen::StartMenu => match self.current_selection {
                Some(CurrentSelection::NewGame) => {
                    self.current_selection = Some(CurrentSelection::Exit)
                }
                Some(CurrentSelection::Exit) => {
                    self.current_selection = Some(CurrentSelection::NewGame)
                }
                None => (),
            },
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn render_main_menu(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" PONG ".bold());
        let instructions = Line::from(vec![
            " Move: ".into(),
            "<Up>/<Down>".blue().bold(),
            " Choose: ".into(),
            "<Enter>".blue().bold(),
            " Quit: ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        let mut lines = vec![];

        lines.push(Line::from(vec![
            Span::raw(
                if matches!(self.current_selection, Some(CurrentSelection::NewGame)) {
                    "◉ "
                } else {
                    "  "
                },
            ),
            Span::styled("New Game", Style::default().fg(Color::Yellow)),
        ]));

        // Dynamically style "Exit"
        lines.push(Line::from(vec![
            Span::raw(
                if matches!(self.current_selection, Some(CurrentSelection::Exit)) {
                    "◉ "
                } else {
                    "  "
                },
            ),
            Span::styled("Exit", Style::default().fg(Color::Yellow)),
        ]));

        let main_menu = Text::from(lines);

        Paragraph::new(main_menu)
            .alignment(Alignment::Center)
            .centered()
            .block(block)
            .render(area, buf);
    }

    fn render_game(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("PONG");
        let instructions = Line::from(vec!["Main Menu:".into(), "<q>".blue().bold()]);
        let instructions_p1 = Line::from(vec![" Move:".into(), "<w>/<s>".yellow().bold()]);
        let instructions_p2 = Line::from(vec![" Move:".into(), "<Up>/<Down>".green().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions_p1.left_aligned())
            .title_bottom(instructions_p2.right_aligned())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        let play_area = Text::from(vec![Line::from("This is the play area")]);
        Paragraph::new(play_area)
            .alignment(Alignment::Center)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.current_screen {
            CurrentScreen::InGame => self.render_game(area, buf),
            CurrentScreen::StartMenu => self.render_main_menu(area, buf),
        }
    }
}

