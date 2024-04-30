use rahjong::{
    draw::Draw,
    hand::Hand,
    tile::{Dragon, Honer, Suit, SuitKind, TileFace, TileId, Wind},
};
use ratatui::{
    prelude::Text,
    style::{Color, Modifier, Style, Stylize},
    text::Span,
    widgets::StatefulWidget,
};
pub struct JpHandWidget<'a> {
    hand: &'a Hand,
    selected: Option<TileId>,
    draw: Option<Draw>,
    doras: Vec<TileId>,
}

impl<'a> JpHandWidget<'a> {
    pub fn new(
        hand: &'a Hand,
        doras: Vec<TileId>,
        selected: Option<TileId>,
        draw: Option<Draw>,
    ) -> Self {
        Self {
            hand,
            doras,
            selected,
            draw,
        }
    }
}

impl<'a> StatefulWidget for JpHandWidget<'a> {
    type State = &'a Hand;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        todo!()
    }
}

impl<'a> ratatui::widgets::Widget for JpHandWidget<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let hand = self.hand;
        let mut x = area.x;
        let y = area.y;
        let span = |face: TileFace| -> Span {
            if let Some(honer) = face.try_into_honer() {
                match honer {
                    Honer::Wind(Wind::East) => {
                        Span::styled("[東]", Style::default().fg(Color::White))
                    }
                    Honer::Wind(Wind::South) => {
                        Span::styled("[南]", Style::default().fg(Color::White))
                    }
                    Honer::Wind(Wind::West) => {
                        Span::styled("[西]", Style::default().fg(Color::White))
                    }
                    Honer::Wind(Wind::North) => {
                        Span::styled("[北]", Style::default().fg(Color::White))
                    }
                    Honer::Dragon(Dragon::Red) => {
                        Span::styled("[中]", Style::default().fg(Color::Magenta))
                    }
                    Honer::Dragon(Dragon::Green) => {
                        Span::styled("[發]", Style::default().fg(Color::Magenta))
                    }
                    Honer::Dragon(Dragon::White) => {
                        Span::styled("[  ]", Style::default().fg(Color::Magenta))
                    }
                }
            } else if let Some(suit) = face.try_into_suit() {
                match suit {
                    Suit {
                        kind: SuitKind::Bamboo,
                        num,
                    } => Span::styled(format!("[{}S]", num), Style::default().fg(Color::Green)),
                    Suit {
                        kind: SuitKind::Character,
                        num,
                    } => Span::styled(format!("[{}M]", num), Style::default().fg(Color::Red)),
                    Suit {
                        kind: SuitKind::Dot,
                        num,
                    } => Span::styled(format!("[{}P]", num), Style::default().fg(Color::Blue)),
                }
            } else {
                Span::styled("[??]", Style::default().fg(Color::Black).bg(Color::White))
            }
        };
        for tile in &hand.tiles {
            let face = tile.face();
            let mut code = span(face);
            if self.selected == Some(tile) {
                code = code.add_modifier(Modifier::UNDERLINED)
            }
            buf.set_span(x, y, &code, 4);
            x += 4;
        }
        if let Some(draw) = self.draw {
            x += 2;
            let face = draw.tile().face();
            let mut code = span(face);
            code = code.add_modifier(Modifier::BOLD);
            buf.set_span(x, y, &code, 4);
        }
    }
}
