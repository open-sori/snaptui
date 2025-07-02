use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, List, ListItem, Padding, Clear},
    style::{Style, Color},
};
use crate::ui::{AppState, utils::apply_margin, PanelFocus};
use super::GroupDetailsFocus;

pub fn draw_group_details(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status_data = app_state.status_data.lock().unwrap();
    let selected_index = app_state.selected_index.lock().unwrap();
    let group_focused_field = app_state.group_focused_field.lock().unwrap();
    let focused_panel = app_state.focused_panel.lock().unwrap();
    let is_editing_group_stream = *app_state.is_editing_group_stream.lock().unwrap();
    let is_editing_group_muted = *app_state.is_editing_group_muted.lock().unwrap();
    let is_editing_group_clients = *app_state.is_editing_group_clients.lock().unwrap();
    let is_editing_name = *app_state.is_editing_group_name.lock().unwrap();
    let margin = 1;

    let is_details_focused = *focused_panel == PanelFocus::Details;
    let title = if is_details_focused { " [ * Group Details * ] " } else { " [ Group Details ] " };

    if let Some(data) = &*status_data {
        if data.result.server.groups.len() > *selected_index {
            let group = &data.result.server.groups[*selected_index];

            let mut details = Vec::new();

            details.push(ListItem::new(format!("  Id: {}", group.id))
                .style(Style::default().fg(Color::White)));

            let name_text = if *group_focused_field == GroupDetailsFocus::Name && is_details_focused {
                if is_editing_name {
                    let editing_name = app_state.editing_group_name.lock().unwrap();
                    let cursor_visible = app_state.cursor_visible.lock().unwrap();
                    let cursor = if *cursor_visible { "_" } else { " " };
                    format!("> Name: {}{}", *editing_name, cursor)
                } else {
                    format!("> Name: {}", group.name)
                }
            } else {
                format!("  Name: {}", group.name)
            };
            let name_style = if *group_focused_field == GroupDetailsFocus::Name && is_details_focused {
                Style::default().fg(Color::Yellow).bold()
            } else {
                Style::default().fg(Color::White)
            };
            details.push(ListItem::new(name_text).style(name_style));

            let stream_id_text = if *group_focused_field == GroupDetailsFocus::StreamId && is_details_focused {
                format!("> Stream Id: {}", group.stream_id)
            } else {
                format!("  Stream Id: {}", group.stream_id)
            };
            let stream_id_style = if *group_focused_field == GroupDetailsFocus::StreamId && is_details_focused {
                Style::default().fg(Color::Yellow).bold()
            } else {
                Style::default().fg(Color::White)
            };
            details.push(ListItem::new(stream_id_text).style(stream_id_style));

            let muted_text = if *group_focused_field == GroupDetailsFocus::Muted && is_details_focused {
                format!("> Muted: {}", group.muted)
            } else {
                format!("  Muted: {}", group.muted)
            };
            let muted_style = if *group_focused_field == GroupDetailsFocus::Muted && is_details_focused {
                Style::default().fg(Color::Yellow).bold()
            } else {
                Style::default().fg(Color::White)
            };
            details.push(ListItem::new(muted_text).style(muted_style));

            let clients_text = if *group_focused_field == GroupDetailsFocus::Clients && is_details_focused {
                "> Clients:".to_string()
            } else {
                "  Clients:".to_string()
            };
            let clients_style = if *group_focused_field == GroupDetailsFocus::Clients && is_details_focused {
                Style::default().fg(Color::Yellow).bold()
            } else {
                Style::default().fg(Color::White)
            };
            details.push(ListItem::new(clients_text).style(clients_style));

            for client in &group.clients {
                details.push(ListItem::new(format!("  - Id: {}", client.id)));
                details.push(ListItem::new(format!("    Connected: {}", client.connected)));
            }

            let list = List::new(details)
                .block(Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow))
                    .padding(Padding::new(3, 3, 1, 1))
                    .title_style(Style::default().fg(Color::Yellow)))
                .style(Style::default().fg(Color::White));

            let inner_area = apply_margin(area, margin);
            f.render_widget(list, inner_area);

            if is_editing_group_stream {
                let stream_selection_index = *app_state.stream_selection_index.lock().unwrap();

                let items: Vec<ListItem> = if let Some(data) = &*status_data {
                    data.result
                        .server
                        .streams
                        .iter()
                        .enumerate()
                        .map(|(i, stream)| {
                            let content = if i == stream_selection_index {
                                format!("> {}", stream.id)
                            } else {
                                format!("  {}", stream.id)
                            };
                            ListItem::new(content).style(if i == stream_selection_index {
                                Style::default().fg(Color::Yellow).bold()
                            } else {
                                Style::default().fg(Color::White)
                            })
                        })
                        .collect()
                } else {
                    vec![ListItem::new("No streams available")]
                };

                let list = List::new(items).block(
                    Block::default()
                        .title(" [ Select Stream ] ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Cyan))
                        .title_style(Style::default().fg(Color::Cyan)),
                );

                let popup_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(30),
                            Constraint::Percentage(40),
                            Constraint::Percentage(30),
                        ]
                    )
                    .split(f.area());

                let popup_area = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(30), Constraint::Percentage(40), Constraint::Percentage(30)]).split(popup_layout[1])[1];
                f.render_widget(Clear, popup_area);
                f.render_widget(list, popup_area);
            }

            if is_editing_group_muted {
                let selection_index = *app_state.group_muted_selection_index.lock().unwrap();
                let options = vec!["true", "false"];

                let items: Vec<ListItem> = options.iter().enumerate().map(|(i, &opt)| {
                    let content = if i == selection_index {
                        format!("> {}", opt)
                    } else {
                        format!("  {}", opt)
                    };
                    ListItem::new(content).style(if i == selection_index {
                        Style::default().fg(Color::Yellow).bold()
                    } else {
                        Style::default().fg(Color::White)
                    })
                }).collect();

                let list = List::new(items).block(
                    Block::default()
                        .title(" [ Select Muted Status ] ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Cyan))
                        .title_style(Style::default().fg(Color::Cyan)),
                );

                let popup_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(40),
                            Constraint::Length(4),
                            Constraint::Percentage(40),
                        ]
                    )
                    .split(f.area());

                let popup_area = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(40), Constraint::Percentage(20), Constraint::Percentage(40)]).split(popup_layout[1])[1];
                f.render_widget(Clear, popup_area);
                f.render_widget(list, popup_area);
            }

            if is_editing_group_clients {
                let client_selection_index = *app_state.client_selection_index.lock().unwrap();
                let selected_clients = app_state.selected_clients.lock().unwrap();

                let all_clients: Vec<_> = status_data.as_ref().unwrap().result.server.groups.iter().flat_map(|g| &g.clients).collect();

                let items: Vec<ListItem> = all_clients
                    .iter()
                    .enumerate()
                    .map(|(i, client)| {
                        let is_selected = selected_clients.contains(&client.id);
                        let content = format!(
                            "{} [{}] {}",
                            if i == client_selection_index { ">" } else { " " },
                            if is_selected { "x" } else { " " },
                            client.id
                        );
                        ListItem::new(content).style(if i == client_selection_index {
                            Style::default().fg(Color::Yellow).bold()
                        } else {
                            Style::default().fg(Color::White)
                        })
                    })
                    .collect();

                let list = List::new(items).block(
                    Block::default()
                        .title(" [ Select Clients ] ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Cyan))
                        .title_style(Style::default().fg(Color::Cyan)),
                );

                let popup_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(30),
                            Constraint::Percentage(40),
                            Constraint::Percentage(30),
                        ]
                    )
                    .split(f.area());

                let popup_area = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(20), Constraint::Percentage(60), Constraint::Percentage(20)]).split(popup_layout[1])[1];
                f.render_widget(Clear, popup_area);
                f.render_widget(list, popup_area);
            }
            return;
        }
    }

    let details = Paragraph::new("Select a group to see details")
        .block(Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .padding(Padding::new(3, 3, 1, 1))
            .title_style(Style::default().fg(Color::Yellow)))
        .style(Style::default().fg(Color::White));

    let inner_area = apply_margin(area, margin);
    f.render_widget(details, inner_area);
}