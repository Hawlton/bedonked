use ratatui::{prelude::*, symbols::line, widgets::*};
use rand::{prelude::*, random_range};

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
    pub grid: Vec<Vec<u8>>
}

impl App {
    pub fn new() -> Self {
        Self {
            score: 0,
            high_score: 0,
            current_screen: CurrentScreen::MainMenu,
            grid: generate_grid()
        }
    }
}

pub fn generate_grid() -> Vec<Vec<u8>> {
    let mut rng = rand::rng();
    let mut grid = vec![vec![0u8]; 10];
    for row in 0..10 {
        for col in 0..10 {
            grid[row][col] = rng.random_range(0..5);
                
        }
    }
    return grid;
}

fn get_emoji(id: u8) -> &'static str {
    match id {
        0 => "🫏",
        1 => "🐮",
        2 => "🍩",
        3 => "🐖",
        4 => "🐄",
        5 => "🐐",
        _ => ""
    }
}

pub fn draw(f: &mut Frame, app: &mut App) {
    match app.current_screen {
        CurrentScreen::MainMenu => draw_main_menu(f),
        CurrentScreen::Game => draw_game(f, app),
        CurrentScreen::GameOver => draw_game_over(f, app),
    }
}

pub fn draw_main_menu(f: &mut Frame) {
    let title = Paragraph::new("Bedonked").style(Style::default().add_modifier(Modifier::BOLD)).block(Block::default().borders(Borders::ALL).title("Bedonked"));
    f.render_widget(title, f.area());
}

pub fn draw_game(f: &mut Frame, app: &mut App) {
    let rows = Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(12), Constraint::Min(0)].as_ref()).split(f.area());
    let mut lines = Vec::new();
    for row in &app.grid{
        let mut cols = Vec::new();
        for &cell in row{
            cols.push(Span::raw(format!("{} ", get_emoji(cell))));
        }
        lines.push(cols);
        
    } 
    let line_block = Paragraph::new(lines).block(Block::default().title("bedonked").borders(Borders::ALL)).alignment(Alignment::Center);
    
    //let main_block = Block::default().borders(Borders::ALL).title("Bedonked");
    let score_block = Paragraph::new(format!("Score: {}", app.score)).block(Block::default().borders(Borders::ALL).title("Score"));
    f.render_widget(line_block, rows[0]);
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