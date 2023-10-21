use std::{collections::HashMap, time::Duration};

use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
use tui_input::Input;

use super::{Component, Frame};
use crate::{
    action::Action,
    config::{Config, KeyBindings},
    tab::Tab,
};

const SHUTTLE_LOGO_ONLY: &str = "
                   .... 
                 :-----.
              .-==-----.
        -+==========--. 
      -+++++=======-.   
         :++++=====     
         :++++++===     
     -----::::=+++-     
     ****=    -+-.      
     ++++-    :.        
";

const SHUTTLE_LOGO: &str = "
                   ....                                                                
                 :-----.                                                               
              .-==-----.            .==                 .      .     -=-               
        -+==========--.        .    :==                ==:    ==:    -=-               
      -+++++=======-.        ==--=  :======.  ==. .==  =====. =====. -=-  .-====-      
         :++++=====         .==-:   :==  -=-  ==. .==  ==:    ==:    -=-  ==-::==-     
         :++++++===           .:==: :==  :=-  ==. .==  ==:    ==-    -=-  ==-.....     
     -----::::=+++-         :==-==: :==  :=-  :======  .====: .====: .==- .-=====      
     ****=    -+-.                                                                     
     ++++-    :.                                             Build Backends. Fast.     
";

#[derive(Default)]
pub struct Home {
    show_help: bool,
    pub action_tx: Option<UnboundedSender<Action>>,
    config: Config,
    pub input: Input,
    pub text: Vec<String>,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Home {
    fn assigned_tab(&self) -> Option<Tab> {
        Some(Tab::Home)
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {}
            Action::ToggleShowHelp => self.show_help = !self.show_help,
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let area = Layout::default()
            .constraints(vec![Constraint::Percentage(100)])
            .margin(3)
            .split(area)[0];
        let rect = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(13), Constraint::Percentage(100)])
            .split(area);
        f.render_widget(
            Paragraph::new(if area.width < 80 {
                SHUTTLE_LOGO_ONLY
            } else {
                SHUTTLE_LOGO
            })
            .style(Style::default().fg(Color::Rgb(253, 145, 62)))
            .alignment(Alignment::Center),
            rect[0],
        );

        let rect = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(100), Constraint::Min(1)])
            .split(rect[1]);

        {
            let rect = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ])
                .split(rect[0]);
            let rect = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Min(1), Constraint::Percentage(100)])
                .split(rect[1]);
            f.render_widget(
                Paragraph::new("Quick Access")
                    .alignment(Alignment::Center)
                    .white(),
                rect[0],
            );
            f.render_widget(Block::default().borders(Borders::ALL), rect[1]);
        }
        f.render_widget(
            Paragraph::new(env!("CARGO_PKG_REPOSITORY"))
                .alignment(Alignment::Center)
                .italic()
                .white(),
            rect[1],
        );
        if self.show_help {
            let rect = f.size().inner(&Margin {
                horizontal: 4,
                vertical: 2,
            });
            f.render_widget(Clear, rect);
            let block = Block::default()
                .title(Line::from(vec![Span::styled(
                    "Key Bindings",
                    Style::default().add_modifier(Modifier::BOLD),
                )]))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow));
            f.render_widget(block, rect);
            let rows = vec![
                Row::new(vec!["j", "Increment"]),
                Row::new(vec!["k", "Decrement"]),
                Row::new(vec!["/", "Enter Input"]),
                Row::new(vec!["ESC", "Exit Input"]),
                Row::new(vec!["Enter", "Submit Input"]),
                Row::new(vec!["q", "Quit"]),
                Row::new(vec!["?", "Open Help"]),
            ];
            let table = Table::new(rows)
                .header(
                    Row::new(vec!["Key", "Action"])
                        .bottom_margin(1)
                        .style(Style::default().add_modifier(Modifier::BOLD)),
                )
                .widths(&[Constraint::Percentage(10), Constraint::Percentage(90)])
                .column_spacing(1);
            f.render_widget(
                table,
                rect.inner(&Margin {
                    vertical: 4,
                    horizontal: 2,
                }),
            );
        };
        Ok(())
    }
}
