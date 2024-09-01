use crate::ui::{areas::Area, State};
use ratatui::crossterm::event::{self, KeyCode, KeyEvent};

pub fn handle_event(state: &mut State, key_event: &KeyEvent) {
    if key_event.kind != event::KeyEventKind::Press {
        return;
    }

    match key_event.code {
        KeyCode::Tab => state.active_area = Area::None,
        KeyCode::BackTab => state.active_area = Area::Variables,
        KeyCode::Char(c) => {
            state.query.text += &c.to_string();
            move_cursor_right(state);
        }
        KeyCode::Backspace => {
            _ = state.query.text.pop();
            move_cursor_left(state);
        }
        KeyCode::Left => {
            move_cursor_left(state);
        }
        KeyCode::Right => {
            move_cursor_right(state);
        }
        KeyCode::Enter => state.query.execute_queue.push(state.query.text.clone()),
        _ => (),
    }
}

fn move_cursor_left(state: &mut State) {
    state.query.cursor_pos = state.query.clamp_cursor(state.query.cursor_pos - 1);
}

fn move_cursor_right(state: &mut State) {
    state.query.cursor_pos = state.query.clamp_cursor(state.query.cursor_pos + 1);
}
