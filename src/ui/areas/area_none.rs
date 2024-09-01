use crate::ui::{areas::Area, State};
use ratatui::crossterm::event::{self, KeyCode, KeyEvent};

pub fn handle_event(state: &mut State, key_event: &KeyEvent) {
    if key_event.kind != event::KeyEventKind::Press {
        return;
    }

    if key_event.code == KeyCode::Char('q') {
        state.should_quit = true;
    } else if key_event.code == KeyCode::Tab {
        state.active_area = Area::Results;
    }
}
