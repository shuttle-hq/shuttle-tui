use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use enum_iterator::{all, cardinality, next_cycle, previous_cycle, Sequence};
use ratatui::{prelude::*, symbols::DOT, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{
    action::Action,
    config::{Config, KeyBindings},
    mode::Mode,
};

#[derive(Default)]
pub struct Tab {
    mode: Mode,
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl Tab {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Tab {
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
                self.mode = next_cycle(&self.mode).unwrap_or_default();
            }
            Action::PreviousTab => {
                self.mode = previous_cycle(&self.mode).unwrap_or_default();
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let modes = all::<Mode>().collect::<Vec<_>>();
        let titles = modes
            .iter()
            .cloned()
            .map(|v| Line::from(v.to_string()))
            .collect();
        let tabs = Tabs::new(titles)
            .block(Block::default().title("Tabs").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(DOT)
            .select(
                modes
                    .iter()
                    .position(|v| v == &self.mode)
                    .unwrap_or_default(),
            );
        f.render_widget(tabs, area);
        Ok(())
    }
}
