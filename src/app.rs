use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use enum_iterator::{next_cycle, previous_cycle, Sequence};
use ratatui::prelude::Rect;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::{
    action::Action,
    args::Args,
    components::{deployments::Deployments, home::Home, projects::Projects, tab::Tabs, Component},
    config::Config,
    shuttle::Shuttle,
    tab::Tab,
    tui,
};

pub struct App {
    pub shuttle: Shuttle,
    pub config: Config,
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub components: Vec<Box<dyn Component>>,
    pub should_quit: bool,
    pub should_suspend: bool,
    pub tab: Tab,
    pub last_tick_key_events: Vec<KeyEvent>,
}

impl App {
    pub fn new(shuttle: Shuttle, args: &Args) -> Result<Self> {
        let config = Config::new()?;
        let tab = Tabs::new();
        let home = Home::new();
        let projects = Projects::new();
        let deployments = Deployments::new();
        Ok(Self {
            shuttle,
            tick_rate: args.tick_rate,
            frame_rate: args.frame_rate,
            components: vec![
                Box::new(tab),
                Box::new(home),
                Box::new(projects),
                Box::new(deployments),
            ],
            should_quit: false,
            should_suspend: false,
            config,
            tab: Tab::Home,
            last_tick_key_events: Vec::new(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let (action_tx, mut action_rx) = mpsc::unbounded_channel();

        let mut tui = tui::Tui::new()?;
        tui.tick_rate(self.tick_rate);
        tui.frame_rate(self.frame_rate);
        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(action_tx.clone())?;
        }

        for component in self.components.iter_mut() {
            component.register_config_handler(self.config.clone())?;
        }

        for component in self.components.iter_mut() {
            component.init()?;
        }

        loop {
            if let Some(e) = tui.next().await {
                match e {
                    tui::Event::Quit => action_tx.send(Action::Quit)?,
                    tui::Event::Tick => action_tx.send(Action::Tick)?,
                    tui::Event::Render => action_tx.send(Action::Render)?,
                    tui::Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
                    tui::Event::Key(key) => {
                        if let Some(keymap) = self.config.keybindings.get(&self.tab) {
                            if let Some(action) = keymap.get(&vec![key.clone()]) {
                                log::info!("Got action: {action:?}");
                                action_tx.send(action.clone())?;
                            } else {
                                // If the key was not handled as a single key action,
                                // then consider it for multi-key combinations.
                                self.last_tick_key_events.push(key);

                                // Check for multi-key combinations
                                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                                    log::info!("Got action: {action:?}");
                                    action_tx.send(action.clone())?;
                                }
                            }
                        };
                    }
                    _ => {}
                }
                for component in self
                    .components
                    .iter_mut()
                    .filter(|v| v.assigned_tab().is_none() || v.assigned_tab() == Some(self.tab))
                {
                    if let Some(action) = component.handle_events(Some(e.clone()))? {
                        action_tx.send(action)?;
                    }
                }
            }

            while let Ok(action) = action_rx.try_recv() {
                if action != Action::Tick && action != Action::Render {
                    log::debug!("{action:?}");
                }
                match action {
                    Action::Tick => {
                        self.last_tick_key_events.drain(..);
                    }
                    Action::Quit => self.should_quit = true,
                    Action::Suspend => self.should_suspend = true,
                    Action::Resume => self.should_suspend = false,
                    Action::Resize(w, h) => {
                        tui.resize(Rect::new(0, 0, w, h))?;
                        tui.draw(|f| {
                            for component in self.components.iter_mut().filter(|v| {
                                v.assigned_tab().is_none() || v.assigned_tab() == Some(self.tab)
                            }) {
                                let r = component.draw(f, f.size());
                                if let Err(e) = r {
                                    action_tx
                                        .send(Action::Error(format!("Failed to draw: {:?}", e)))
                                        .unwrap();
                                }
                            }
                        })?;
                    }
                    Action::Render => {
                        tui.draw(|f| {
                            for component in self.components.iter_mut().filter(|v| {
                                v.assigned_tab().is_none() || v.assigned_tab() == Some(self.tab)
                            }) {
                                let r = component.draw(f, f.size());
                                if let Err(e) = r {
                                    action_tx
                                        .send(Action::Error(format!("Failed to draw: {:?}", e)))
                                        .unwrap();
                                }
                            }
                        })?;
                    }
                    Action::NextTab => {
                        self.tab = next_cycle(&self.tab).unwrap_or_default();
                    }
                    Action::PreviousTab => {
                        self.tab = previous_cycle(&self.tab).unwrap_or_default();
                    }
                    _ => {}
                }
                for component in self
                    .components
                    .iter_mut()
                    .filter(|v| v.assigned_tab().is_none() || v.assigned_tab() == Some(self.tab))
                {
                    if let Some(action) = component.update(action.clone())? {
                        action_tx.send(action)?
                    };
                }
            }
            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
                tui = tui::Tui::new()?;
                tui.tick_rate(self.tick_rate);
                tui.frame_rate(self.frame_rate);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }
}
