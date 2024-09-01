mod area_none;
mod query;
mod result;
mod variables;

use ratatui::crossterm::event::KeyEvent;

#[derive(Copy, Clone)]
pub enum Area {
    None,
    Result,
    Variables,
    Query,
}

use crate::ui::State;

impl Area {
    pub fn handle_event(&self, state: &mut State, key_event: &KeyEvent) {
        match self {
            Area::None => area_none::handle_event(state, key_event),
            Area::Result => result::handle_event(state, key_event),
            Area::Variables => variables::handle_event(state, key_event),
            Area::Query => query::handle_event(state, key_event),
        }
    }
}
