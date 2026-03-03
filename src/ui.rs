use ratatui::{widgets::*, prelude::*};

#[derive(PartialEq)]
pub enum CurrentScreen {
    MainMenu,
    Game,
    GameOver,
}

pub struct App {
    pub score: u32,
    pub high_score: u32,
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> Self {
        Self {
            score: 0,
            high_score: 0,
            current_screen: CurrentScreen::MainMenu,
        }
    }
    
}

pub fn draw(f: &mut Frame, app: &mut App) {
    match app.current_screen {
        CurrentScreen::MainMenu => draw_main_menu(f, app),
        CurrentScreen::Game => draw_game(f, app),
        CurrentScreen::GameOver => draw_game_over(f, app),
    }
}

pub fn draw_main_menu(f: &mut Frame, app: &mut App) {
    let title = Paragraph::new("Bedonked").block(Block::default().borders(Borders::ALL).title("Bedonked"));
    f.render_widget(title, f.size());
}

pub fn draw_game(f: &mut Frame, app: &mut App) {
    let rows = Layout::default().direction(Direction::Vertical).constraints([Constraint::Min(0), Constraint::Max(3)].as_ref()).split(f.size());
    let main_block = Block::default().borders(Borders::ALL).title("Bedonked");
    let score_block = Paragraph::new(format!("Score: {}", app.score)).block(Block::default().borders(Borders::ALL).title("Score"));
    f.render_widget(main_block, rows[0]);
    f.render_widget(score_block, rows[1]);
}

pub fn draw_game_over(f: &mut Frame, app: &mut App) {
    let title = Paragraph::new("Game Over").block(Block::default().borders(Borders::ALL).title("Game Over"));
    let score = Paragraph::new(format!("Final Score: {}", app.score)).block(Block::default().borders(Borders::ALL).title("Final Score"));
    let high_score = Paragraph::new(format!("High Score: {}", app.high_score)).block(Block::default().borders(Borders::ALL).title("High Score"));
    let layout = Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Length(3)].as_ref()).split(f.size());
    f.render_widget(title, layout[0]);
    f.render_widget(score, layout[1]);
    f.render_widget(high_score, layout[2]);
}