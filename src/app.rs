use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use tui_widget_list::ListState;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResourceType {
    Wood,
    Stone,
    Iron,
    Diamond,
}

impl Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ResourceType::Wood => "Wood",
            ResourceType::Stone => "Stone",
            ResourceType::Iron => "Iron",
            ResourceType::Diamond => "Diamond",
        };
        write!(f, "{str}")
    }
}

#[derive(Debug, Clone)]
pub struct Cost {
    pub amount: u64,
    pub resource_type: ResourceType,
}

impl Cost {
    pub fn new(amount: u64, resource_type: ResourceType) -> Self {
        Self {
            amount,
            resource_type,
        }
    }
}

impl Display for Cost {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.amount, self.resource_type)
    }
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub resource_type: ResourceType,
    pub amount: u64,
    pub level: u64,
    pub cost: Cost,
    pub progress: f64,
    pub progress_per_tick: f64,
}

impl Resource {
    pub fn new(name: ResourceType, progress_per_tick: f64, cost: Cost) -> Self {
        Self {
            resource_type: name,
            progress_per_tick,
            cost,
            amount: 0,
            level: 0,
            progress: 0.0,
        }
    }

    pub(crate) fn start_with(self, amount: u64) -> Self {
        Self { amount, ..self }
    }

    pub fn upgrade_cost(&self) -> Cost {
        Cost::new(
            self.cost.amount.pow(self.level as u32 + 1),
            self.cost.resource_type,
        )
    }
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Counter.
    pub resources: Vec<Resource>,
    /// Event handler.
    pub events: EventHandler,
    pub list_state: RefCell<ListState>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            resources: vec![
                Resource::new(ResourceType::Wood, 1.0, Cost::new(2, ResourceType::Wood))
                    .start_with(2),
                Resource::new(ResourceType::Stone, 0.5, Cost::new(3, ResourceType::Wood)),
                Resource::new(ResourceType::Iron, 0.1, Cost::new(4, ResourceType::Stone)),
                Resource::new(
                    ResourceType::Diamond,
                    0.010,
                    Cost::new(5, ResourceType::Iron),
                ),
            ],
            events: EventHandler::new(),
            list_state: RefCell::new(ListState::default()),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => {
                if let ratatui::crossterm::event::Event::Key(key_event) = event {
                    self.handle_key_event(key_event)?
                }
            }
            Event::App(app_event) => match app_event {
                AppEvent::GoDown => self.list_state.borrow_mut().next(),
                AppEvent::GoUp => self.list_state.borrow_mut().previous(),
                AppEvent::Upgrade => {
                    let index = self.list_state.borrow().selected;
                    self.upgrade_resource(index)
                }
                AppEvent::Quit => self.quit(),
            },
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Down => self.events.send(AppEvent::GoDown),
            KeyCode::Up => self.events.send(AppEvent::GoUp),
            KeyCode::Enter => self.events.send(AppEvent::Upgrade),
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&mut self) {
        for resource in &mut self.resources {
            resource.progress += resource.level as f64 * resource.progress_per_tick / 100.0;
            let whole_part = resource.progress.floor() as u64;
            resource.amount += whole_part;
            resource.progress = resource.progress.fract();
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn upgrade_resource(&mut self, index: Option<usize>) {
        let Some(index) = index else {
            self.list_state.borrow_mut().next();
            return;
        };

        let cost = self.resources[index].upgrade_cost();
        let cost_resource = self
            .resources
            .iter_mut()
            .find(|x| x.resource_type == cost.resource_type);
        let Some(cost_resource) = cost_resource else {
            return;
        };

        if cost_resource.amount < cost.amount {
            return;
        }
        cost_resource.amount -= cost.amount;

        self.resources[index].level += 1;
    }
}
