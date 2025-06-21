use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Tabs, Padding},
    layout::{Rect},
    style::{Style, Color, Stylize},
};
use crate::ui::AppState;
use crate::ui::utils::apply_margin;

#[derive(Debug, Clone, PartialEq)]
pub enum TabSelection {
    Groups,
    Clients,
    Streams,
}

#[derive(Debug, Clone)]
pub struct Tab {
    pub title: String,
    pub index: usize,
}

pub fn draw_tabs(f: &mut Frame, app_state: &AppState, area: Rect) {
    let active_tab = app_state.active_tab.lock().unwrap();
    let margin = 1;

    let tabs = vec![
        Tab { title: "Groups".to_string(), index: 0 },
        Tab { title: "Clients".to_string(), index: 1 },
        Tab { title: "Streams".to_string(), index: 2 },
    ];

    let titles = tabs.iter().map(|t| {
        let is_active = *active_tab == match t.index {
            0 => TabSelection::Groups,
            1 => TabSelection::Clients,
            _ => TabSelection::Streams,
        };

        let title = Span::styled(
            format!(" {} ", t.title),
            if is_active {
                Style::default().fg(Color::Blue).bold()
            } else {
                Style::default().fg(Color::White)
            }
        );

        Line::from(title)
    }).collect::<Vec<_>>();

    let tabs = Tabs::new(titles)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" [ Menu ] ")
            .padding(Padding::new(1, 1, 1, 1))
            .border_style(Style::default().fg(Color::Blue))
            .title_style(Style::default().fg(Color::Blue)))
        .style(Style::default().fg(Color::White))
        .select(Some(match *active_tab {
            TabSelection::Groups => 0,
            TabSelection::Clients => 1,
            TabSelection::Streams => 2,
        }))
        .divider(Span::styled(
            " | ",
            Style::default().fg(Color::White)
        ));

    let inner_area = apply_margin(area, margin);
    f.render_widget(tabs, inner_area);
}