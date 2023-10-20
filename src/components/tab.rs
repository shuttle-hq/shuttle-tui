use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use enum_iterator::{all, cardinality, next_cycle, previous_cycle, Sequence};
use ratatui::{
    prelude::Rect,
    style::{Color, Style},
    symbols::DOT,
    text::Line,
    widgets::{Block, Borders, Tabs as TuiTabs},
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
        let modes = all::<Tab>().collect::<Vec<_>>();
        let titles = modes
            .iter()
            .cloned()
            .map(|v| Line::from(v.to_string()))
            .collect();
        let tabs = TuiTabs::new(titles)
            .block(Block::default().title("Tabs").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(DOT)
            .select(
                modes
                    .iter()
                    .position(|v| v == &self.tab)
                    .unwrap_or_default(),
            );
        f.render_widget(tabs, area);
        Ok(())
    }
}
