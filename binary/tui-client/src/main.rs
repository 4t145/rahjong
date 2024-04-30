pub mod widget;

use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rahjong::{player::Player, tile::Wind};
use ratatui::{prelude::*, widgets::*};

use crate::widget::JpHandWidget;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut should_quit = false;
    use rahjong::rules::jp::round::Round;
    let mut round = Round::new(Wind::East);
    round.start();
    let sight = round.player_sight(Player::EAST);
    let hand = sight.self_deck.hand;
    let draw = sight.draw;
    while !should_quit {
        terminal.draw(|frame| {
            let hand_widget = JpHandWidget::new(&hand, vec![], hand.tiles.iter().nth(1), draw);
            frame.render_widget(hand_widget, frame.size());
        })?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        let event = event::read()?;
        match event {
            Event::Key(key) => {
                if key.code == KeyCode::Char('q')
                    && key.modifiers.is_empty()
                    && key.kind == event::KeyEventKind::Press
                {
                    return Ok(true);
                }
            }
            _ => {}
        }
        
    }
    Ok(false)
}
