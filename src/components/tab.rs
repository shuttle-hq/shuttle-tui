use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use enum_iterator::{all, cardinality, next_cycle, previous_cycle, Sequence};
use ratatui::{
    prelude::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::DOT,
    text::Line,
    widgets::{block::Position, Block, Borders, Padding, Tabs as TuiTabs},
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{
    action::Action,
    config::{Config, KeyBindings},
    tab::Tab,
};

#[derive(Default)]
pub struct Tabs {
    tab: Tab,
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl Tabs {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Tabs {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {}
            Action::NextTab => {
                self.tab = next_cycle(&self.tab).unwrap_or_default();
            }
            Action::PreviousTab => {
                self.tab = previous_cycle(&self.tab).unwrap_or_default();
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let rect = Layout::default()
            .constraints(vec![Constraint::Percentage(100)])
            .margin(1)
            .split(area);
        let modes = all::<Tab>().collect::<Vec<_>>();
        let titles = modes
            .iter()
            .cloned()
            .map(|v| Line::from(v.to_string()))
            .collect::<Vec<Line>>();
        f.render_widget(
            Block::default()
                .title("Shuttle TUI")
                .title_position(Position::Top)
                .title_alignment(Alignment::Center)
                .title_style(Style::default().bold())
                .borders(Borders::ALL),
            rect[0],
        );
        let rect = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(
                    (rect[0]
                        .width
                        .checked_sub(titles.iter().map(|v| v.width()).sum::<usize>() as u16 + 10))
                    .unwrap_or(rect[0].width / 2)
                        / 2,
                ),
                Constraint::Percentage(100),
            ])
            .margin(1)
            .split(rect[0]);
        let tabs = TuiTabs::new(titles)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Rgb(253, 145, 62)))
            .divider(DOT)
            .select(
                modes
                    .iter()
                    .position(|v| v == &self.tab)
                    .unwrap_or_default(),
            );
        f.render_widget(tabs, rect[1]);
        Ok(())
    }
}
