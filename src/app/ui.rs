use crate::app::state::AppState;
use crate::app::Actions;
use crate::app::App;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, Wrap};
use tui::Frame;

pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let size = rect.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Percentage(90)])
        .margin(1)
        .split(size);

    let header_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[0]);

    let title = draw_title();
    rect.render_widget(title, header_chunks[0]);

    let help = draw_help(app.actions());
    rect.render_widget(help, header_chunks[1]);

    let body = draw_body(app.state());
    rect.render_widget(body, chunks[1]);
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new(env!("CARGO_PKG_NAME"))
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(Block::default().style(Style::default().fg(Color::White)))
}

fn draw_help(actions: &Actions) -> Table {
    let key_style = Style::default().fg(Color::LightCyan);
    let help_style = Style::default().fg(Color::Gray);

    let mut rows = vec![];
    for action in actions.actions().iter() {
        let keys: Vec<String> = action.keys().iter().map(|k| k.to_string()).collect();
        let key = keys.join(", ");
        let row = Row::new(vec![
            Cell::from(Span::styled(key, key_style)),
            Cell::from(Span::styled(action.to_string(), help_style)),
        ]);
        rows.push(row);
    }

    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        )
        .widths(&[Constraint::Length(20), Constraint::Percentage(80)])
        .column_spacing(1)
}

fn draw_body<'a>(_state: &AppState) -> Paragraph<'a> {
    Paragraph::new("Hello world")
        .alignment(Alignment::Center)
        .block(Block::default())
        .style(Style::default())
        .wrap(Wrap { trim: true })
}
