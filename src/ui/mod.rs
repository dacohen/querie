mod areas;

use areas::Area;
use ratatui::{
    crossterm::event::{self, Event},
    layout::{Constraint, Layout, Position},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Paragraph, Row, Table},
    Frame,
};

use crate::db;

pub struct State {
    active_area: Area,
    query: QueryState,
    results: ResultsState,
    should_quit: bool,
}

struct QueryState {
    text: String,
    cursor_pos: u16,
    execute_queue: Vec<String>,
}

struct ResultsState {
    result_sets: Vec<Vec<Vec<db::DBResult>>>,
    selected_result: usize,
}

impl QueryState {
    fn byte_index(&self) -> usize {
        self.text
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_pos as usize)
            .unwrap_or(self.text.len())
    }

    fn clamp_cursor(&self, new_cursor_pos: u16) -> u16 {
        return new_cursor_pos.clamp(0, self.text.chars().count() as u16);
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            active_area: Area::Query,
            query: QueryState {
                text: String::new(),
                cursor_pos: 0,
                execute_queue: Vec::new(),
            },
            results: ResultsState {
                result_sets: Vec::new(),
                selected_result: 0,
            },
            should_quit: false,
        }
    }
}

impl State {
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn pop_query(&mut self) -> Option<String> {
        self.query.execute_queue.pop()
    }

    pub fn add_results(&mut self, result: Vec<Vec<db::DBResult>>) {
        self.results.result_sets.push(result);
        self.results.selected_result = self.results.result_sets.len() - 1;
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
                Area::Results => "Result",
                Area::Variables => "Variables",
                Area::Query => "Query (ENTER to run)",
            }),
            status_area,
        );

        let results_block = Block::bordered()
            .title("Results")
            .border_type(match self.active_area {
                Area::Results => BorderType::Thick,
                _ => BorderType::Plain,
            })
            .border_style(match self.active_area {
                Area::Results => active_style,
                _ => inactive_style,
            });
        frame.render_widget(&results_block, result_area);

        let results_block_inner_area = results_block.inner(result_area);
        if self.results.result_sets.len() > 0 {
            let result_set = self
                .results
                .result_sets
                .get(self.results.selected_result)
                .unwrap();
            let mut rows_vec: Vec<Row> = Vec::new();
            let mut widths: Vec<Constraint> = Vec::new();
            let mut column_names: Vec<String> = Vec::new();
            let mut first_row = true;

            for row in result_set {
                let mut row_vec: Vec<String> = Vec::new();
                for col in row {
                    if first_row {
                        column_names.push(col.column_name.clone());
                        widths.push(Constraint::Min(col.column_name.len() as u16));
                    }
                    row_vec.push(col.value.clone());
                }
                first_row = false;
                rows_vec.push(Row::new(row_vec));
            }

            let table = Table::new(rows_vec, widths)
                .column_spacing(1)
                .header(Row::new(column_names).bottom_margin(1))
                .block(
                    Block::bordered().title(format!("Result {}", self.results.selected_result + 1)),
                );

            frame.render_widget(table, results_block_inner_area);
        }

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
