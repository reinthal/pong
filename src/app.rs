use std::{fs::File, io::Write};
use std::ops::Add;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    symbols::Marker,
    text::{Line, Span, Text},
    widgets::{
        canvas::{Canvas, Circle, Rectangle},
        Block, Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};
use std::io;
use crate::constants;
use std::time::{Duration, Instant};

#[derive(PartialEq)]
pub enum CurrentScreen {
    StartMenu,
    InGame,
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Na,
}

pub struct Player {
    pub paddle: Rectangle,
    pub lifes: usize,
    pub starting_lifes: usize,
    pub direction: Direction,
}

impl Player {
    pub fn new(x: f64, color: Color) -> Player {
        Player {
            lifes: 3,
            starting_lifes: 3,
            direction: Direction::Na,
            paddle: Rectangle {
                x,
                y: 10.0,
                width: 3.0,
                height: 20.0,
                color
            }
        }
    }
}

pub enum CurrentSelection {
    NewGame,
    Exit,
}

pub struct App {
    pub tick_count: u64,
    pub marker: Marker,
    pub current_screen: CurrentScreen,
    pub current_selection: Option<CurrentSelection>,
    pub exit: bool,
    pub ball: Circle,
    pub playground: Rect,
    vx: f64,
    vy: f64,
    pub p1: Player,
    pub p2: Player,
    pub logfile: File,
}

impl App {
    
    pub fn new() -> App {
        let logfile = File::create("app_log.txt").expect("could not open file");
        App {
            playground: Rect::new(0, 0, 200, 100),
            vx: 1.0,
            vy: 1.0,
            ball: Circle {
                x: 10.0,
                y: 10.0,
                radius: 5.0,
                color: Color::Cyan,
            },
            tick_count: 0,
            marker: Marker::Dot,
            current_screen: CurrentScreen::StartMenu,
            current_selection: Some(CurrentSelection::NewGame),
            exit: false,
            p1: Player::new(10.0,Color::Yellow),
            p2: Player::new(190.0,Color::Green),
            logfile,
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;

        if self.current_screen == CurrentScreen::StartMenu {
            return;
        }
        assert!(self.current_screen == CurrentScreen::InGame);
        // Move Paddles
        match self.p1.direction {
            Direction::Down => self.p1.paddle.y = f64::max(self.p1.paddle.y - 1.0, 0.0),
            Direction::Up => self.p1.paddle.y = f64::min(self.p1.paddle.y + 1.0, 100.0 - self.p1.paddle.height),
            Direction::Na => (),
        }

        match self.p2.direction {
            Direction::Down => self.p2.paddle.y = f64::max(self.p2.paddle.y - 1.0, 0.0),
            Direction::Up => self.p2.paddle.y = f64::min(self.p2.paddle.y + 1.0, 100.0 - self.p2.paddle.height),
            Direction::Na => (),
        }

        // bounce the ball by flipping the velocity vector
        let ball = &self.ball;
        let playground = self.playground;

        if ball.x - ball.radius < f64::from(playground.left()) {
            self.p1.lifes -= 1;
            if self.p1.lifes < 1 {
                // P2 Wins!
                self.exit = true; // TODO Implement P2 Wins!
            } else {
                // Reset game  and wait 1000ms
                self.vx = -self.vx;  // TODO: implement reset
            }
        }

        if ball.x + ball.radius > f64::from(playground.right()) {
            self.p2.lifes -= 1;
            if self.p2.lifes < 1 {
                // P1 Wins!
                self.exit = true; // TODO Implement p1 wins!
            } else {
                // reset game and wait 1000ms
                self.vx = -self.vx;  // TODO: implement reset

            }
        }
        

        // Implement paddle bounce
        if (ball.x - ball.radius < f64::from(self.p1.paddle.x)) && 
        (self.p1.paddle.y < ball.y && ball.y < self.p1.paddle.y + self.p1.paddle.height) {
            self.vx = -self.vx;
        }
        if ball.x + ball.radius > f64::from(self.p2.paddle.x) &&
        (self.p2.paddle.y < ball.y && ball.y < self.p2.paddle.y + self.p1.paddle.height) {
            self.vx = -self.vx;
        }
        
        if ball.y - ball.radius < f64::from(playground.top())
            || ball.y + ball.radius > f64::from(playground.bottom())
        {
            self.vy = -self.vy;
        }
        self.ball.x += self.vx;
        self.ball.y += self.vy;
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                self.handle_events()?;
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.logfile.write_all(b"Press event happened!\n")?;
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
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
                KeyCode::Char('w') => self.p1.direction = if self.p1.direction == Direction::Down {
                    Direction::Na
                } else {
                    Direction::Up
                },
                KeyCode::Char('s') => self.p1.direction = if self.p1.direction == Direction::Up {
                    Direction::Na
                } else {
                    Direction::Down
                },
                // Move p2
                KeyCode::Up => self.p2.direction = if self.p2.direction == Direction::Down {
                    Direction::Na
                } else {
                    Direction::Up
                },
                KeyCode::Down => self.p2.direction = if self.p2.direction == Direction::Up {
                    Direction::Na
                } else {
                    Direction::Down
                },
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
        
        let mut lifes = String::new();
        for _i in 0..self.p1.lifes {
            lifes.push('◉');
        }
        let p1_lifes = Line::from(vec![lifes.into()]);
        
        lifes = String::new();
        for _i in 0..self.p2.lifes {
            lifes.push('◉');
        }
        let p2_lifes = Line::from(vec![lifes.into()]);
        
        // 1. Create the block that surrounds the game area
        let instructions = Line::from(vec!["Main Menu:".into(), "<q>".blue().bold()]);
        let instructions_p1 = Line::from(vec![" Move:".into(), "<w>/<s>".yellow().bold()]);
        let instructions_p2 = Line::from(vec![" Move:".into(), "<Up>/<Down>".green().bold()]);
        let block = Block::bordered()
        .title(Line::from("PONG").centered())
        .title(p1_lifes.left_aligned())
        .title(p2_lifes.right_aligned())
        .title_bottom(instructions_p1.left_aligned())
        .title_bottom(instructions_p2.right_aligned())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);
    
        // 2. Split the game's renderable area by accounting for the block's margins
        let inner_area = block.inner(area); // The area inside the bordered block

        // 3. Define the bounds for the Canvas (x, y ranges correspond to the playground)
        let x_bounds = [
            self.playground.left() as f64,
            self.playground.right() as f64,
        ];
        let y_bounds = [
            self.playground.top() as f64,
            self.playground.bottom() as f64,
        ];

        // 4. Create the canvas and draw the ball
        let canvas = Canvas::default()
            .block(block) // Attach the block
            .marker(self.marker)
            .paint(|ctx| {
                ctx.draw(&self.ball); // Draw the ball at its current position
                ctx.draw(&self.p1.paddle);
                ctx.draw(&self.p2.paddle);

            })
            .x_bounds(x_bounds)
            .y_bounds(y_bounds);

        // 5. Render the canvas within the game's area
        canvas.render(inner_area, buf);
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
