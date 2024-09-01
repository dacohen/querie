use crate::ui::{areas::Area, State};
use ratatui::crossterm::event::{self, KeyCode, KeyEvent};

pub fn handle_event(state: &mut State, key_event: &KeyEvent) {
    if key_event.kind != event::KeyEventKind::Press {
        return;
    }

    match key_event.code {
        KeyCode::Tab => state.active_area = Area::Variables,
        KeyCode::BackTab => state.active_area = Area::None,
        KeyCode::PageUp => {
            state.results.selected_result = state
                .results
                .selected_result
                .saturating_add(1)
                .clamp(0, state.results.result_sets.len() - 1);
        }
        KeyCode::PageDown => {
            state.results.selected_result = state
                .results
                .selected_result
                .saturating_sub(1)
                .clamp(0, state.results.result_sets.len() - 1);
        }
        _ => (),
    }
}
