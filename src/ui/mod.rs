mod areas;

use areas::Area;
use ratatui::{
    crossterm::event::{self, Event},
    layout::{Constraint, Layout, Position},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

pub struct State {
    active_area: Area,
    query: QueryState,
    should_quit: bool,
}

struct QueryState {
    text: String,
    cursor_pos: u16,
    execute_queue: Vec<String>,
}

impl QueryState {
    fn clamp_cursor(&self, new_cursor_pos: u16) -> u16 {
        return new_cursor_pos.clamp(0, self.text.chars().count() as u16);
    }
}

impl State {
    pub fn new() -> Self {
        return State {
            active_area: Area::None,
            query: QueryState {
                text: String::new(),
                cursor_pos: 0,
                execute_queue: Vec::new(),
            },
            should_quit: false,
        };
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn pop_query(&mut self) -> Option<String> {
        self.query.execute_queue.pop()
    }

    pub fn handle_event(&mut self) {
        if event::poll(std::time::Duration::from_millis(50)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                let active_area = self.active_area;
                active_area.handle_event(self, &key_event);
            }
        }
    }

    pub fn ui(&self, frame: &mut Frame) {
        let active_style = Style::new().yellow();
        let inactive_style = Style::new().white();

        let [title_area, main_area, query_area, status_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Percentage(75),
            Constraint::Percentage(25),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        let [result_area, variable_area] =
            Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)])
                .areas(main_area);

        frame.render_widget(Block::new().title("Querie"), title_area);

        frame.render_widget(
            Block::bordered().title(match self.active_area {
                Area::None => "- (q to quit)",
                Area::Result => "Result",
                Area::Variables => "Variables",
                Area::Query => "Query (ENTER to run)",
            }),
            status_area,
        );

        frame.render_widget(
            Block::bordered()
                .title("Result Area")
                .border_type(match self.active_area {
                    Area::Result => BorderType::Thick,
                    _ => BorderType::Plain,
                })
                .border_style(match self.active_area {
                    Area::Result => active_style,
                    _ => inactive_style,
                }),
            result_area,
        );
        frame.render_widget(
            Block::bordered()
                .title("Variables")
                .border_type(match self.active_area {
                    Area::Variables => BorderType::Thick,
                    _ => BorderType::Plain,
                })
                .border_style(match self.active_area {
                    Area::Variables => active_style,
                    _ => inactive_style,
                }),
            variable_area,
        );
        frame.render_widget(
            Paragraph::new(self.query.text.to_owned()).block(
                Block::bordered()
                    .title("Query (Insert)")
                    .border_type(match self.active_area {
                        Area::Query => BorderType::Thick,
                        _ => BorderType::Plain,
                    })
                    .border_style(match self.active_area {
                        Area::Query => active_style,
                        _ => inactive_style,
                    }),
            ),
            query_area,
        );

        match self.active_area {
            Area::Query => {
                frame.set_cursor_position(Position::new(
                    query_area.x + self.query.cursor_pos + 1,
                    query_area.y + 1,
                ));
            }
            _ => (),
        }
    }
}
