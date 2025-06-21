use crate::commands::server::getstatus::{create_status_request, extract_server_version};
use crate::core::input::events::{
    get_next_client_field, get_next_group_field, get_previous_client_field,
    get_previous_group_field, handle_input,
};
use crate::core::input::InputEvent;
use crate::core::websocket::ConnectionStatus;
use crate::models::client::{
    onconnect::ClientOnConnect, ondisconnect::ClientOnDisconnect,
    onlatencychanged::ClientOnLatencyChanged, onnamechanged::ClientOnNameChanged,
    onvolumechanged::ClientOnVolumeChanged,
};
use crate::models::group::{
    onmute::GroupOnMute, onnamechanged::GroupOnNameChanged, onstreamchanged::GroupOnStreamChanged,
};
use crate::models::server::{getstatus::GetStatusData, onupdate::ServerOnUpdate};
use crate::models::stream::{onproperties::StreamOnProperties, onupdate::StreamOnUpdate};
use crate::ui::{
    draw_ui, initialize_terminal, restore_terminal, AppState, ClientDetailsFocus,
    GroupDetailsFocus, PanelFocus, TabSelection,
};
use chrono::Local;
use serde_json::Value;
use std::collections::HashMap;
use std::io::Result;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

pub struct Application {
    pub terminal: ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    pub app_state: AppState,
    pub status_data: Arc<Mutex<Option<GetStatusData>>>,
    pub cmd_tx: mpsc::Sender<String>,
}

impl Application {
    pub fn new(cmd_tx: mpsc::Sender<String>) -> Result<Self> {
        let terminal = initialize_terminal()?;
        let status_data = Arc::new(Mutex::new(None));
        let app_state = AppState {
            events: Arc::new(Mutex::new(Vec::new())),
            status: Arc::new(Mutex::new(ConnectionStatus::Disconnected)),
            server_version: Arc::new(Mutex::new(String::new())),
            status_data: Arc::clone(&status_data),
            active_tab: Arc::new(Mutex::new(TabSelection::Groups)),
            selected_index: Arc::new(Mutex::new(0)),
            focused_panel: Arc::new(Mutex::new(PanelFocus::List)),
            events_scroll_offset: Arc::new(Mutex::new(0)),
            group_focused_field: Arc::new(Mutex::new(GroupDetailsFocus::None)),
            client_focused_field: Arc::new(Mutex::new(ClientDetailsFocus::None)),
            is_editing_group_name: Arc::new(Mutex::new(false)),
            editing_group_name: Arc::new(Mutex::new(String::new())),
            is_editing_client_name: Arc::new(Mutex::new(false)),
            editing_client_name: Arc::new(Mutex::new(String::new())),
            is_editing_client_volume: Arc::new(Mutex::new(false)),
            editing_client_volume: Arc::new(Mutex::new(String::new())),
            is_editing_group_stream: Arc::new(Mutex::new(false)),
            stream_selection_index: Arc::new(Mutex::new(0)),
            is_editing_group_muted: Arc::new(Mutex::new(false)),
            group_muted_selection_index: Arc::new(Mutex::new(0)),
            is_editing_client_muted: Arc::new(Mutex::new(false)),
            client_muted_selection_index: Arc::new(Mutex::new(0)),
            is_editing_group_clients: Arc::new(Mutex::new(false)),
            selected_clients: Arc::new(Mutex::new(Vec::new())),
            client_selection_index: Arc::new(Mutex::new(0)),
            is_editing_client_latency: Arc::new(Mutex::new(false)),
            editing_client_latency: Arc::new(Mutex::new(String::new())),
            cursor_visible: Arc::new(Mutex::new(true)),
            last_cursor_toggle: Arc::new(Mutex::new(Instant::now())),
            request_methods: Arc::new(Mutex::new(HashMap::new())),
        };

        Ok(Self {
            terminal,
            app_state,
            status_data,
            cmd_tx,
        })
    }

    pub async fn run(
        &mut self,
        mut message_rx: mpsc::Receiver<String>,
        mut status_rx: mpsc::Receiver<ConnectionStatus>,
    ) -> Result<()> {
        let status_arc: Arc<Mutex<ConnectionStatus>> = Arc::clone(&self.app_state.status);
        tokio::spawn(async move {
            while let Some(new_status) = status_rx.recv().await {
                if let Ok(mut status) = status_arc.lock() {
                    *status = new_status;
                }
            }
        });

        let status_data_arc: Arc<Mutex<Option<GetStatusData>>> = Arc::clone(&self.status_data);
        let events_arc = Arc::clone(&self.app_state.events);
        let version_arc = Arc::clone(&self.app_state.server_version);
        let request_methods_arc = Arc::clone(&self.app_state.request_methods);
        let cmd_tx_clone = self.cmd_tx.clone();

        tokio::spawn(async move {
            while let Some(msg) = message_rx.recv().await {
                if let Ok(json_value) = serde_json::from_str::<Value>(&msg) {
                    if let Some(result) = json_value.get("result") {
                        if result.get("server").is_some() {
                            if let Ok(status) =
                                crate::commands::server::getstatus::parse_status_response(&msg)
                            {
                                if let Ok(mut data) = status_data_arc.lock() {
                                    *data = Some(status.clone());
                                }
                                if let Some(version) = extract_server_version(&msg) {
                                    if let Ok(mut version_lock) = version_arc.lock() {
                                        *version_lock = version;
                                    }
                                }
                            }
                        } else {
                            let mut method_name = "Response".to_string();
                            if let Some(id_val) = json_value.get("id") {
                                if let Some(id_str) = id_val.as_str() {
                                    let mut methods = request_methods_arc.lock().unwrap();
                                    if let Some(method) = methods.remove(id_str) {
                                        method_name = method;
                                    }
                                }
                            }

                            {
                                let mut events = events_arc.lock().unwrap();
                                let now = Local::now();
                                let formatted_time = now.format("%Y-%m-%d %H:%M:%S");
                                let event_string = format!(
                                    "{} - {}:\n{}",
                                    formatted_time, method_name, msg
                                );
                                events.push(event_string);
                            }

                            let status_request = create_status_request();
                            if let Err(_) = cmd_tx_clone.send(status_request).await {}
                        }
                    } else if let Some(method_val) = json_value.get("method") {
                        if let Some(method_str) = method_val.as_str() {
                            let mut events = events_arc.lock().unwrap();
                            let now = Local::now();
                            let formatted_time = now.format("%Y-%m-%d %H:%M:%S");

                            let event_string = match method_str {
                                "Client.OnConnect" => {
                                    match serde_json::from_str::<ClientOnConnect>(&msg) {
                                        Ok(n) => {
                                            let params_str =
                                                serde_json::to_string(&n.params).unwrap_or_default();
                                            Some(format!(
                                                "{} - {}\n{}",
                                                formatted_time, n.method, params_str
                                            ))
                                        }
                                        Err(e) => Some(format!(
                                            "Error parsing Client.OnConnect: {}",
                                            e
                                        )),
                                    }
                                }
                                "Client.OnDisconnect" => {
                                    match serde_json::from_str::<ClientOnDisconnect>(&msg) {
                                        Ok(n) => {
                                            let params_str =
                                                serde_json::to_string(&n.params).unwrap_or_default();
                                            Some(format!(
                                                "{} - {}\n{}",
                                                formatted_time, n.method, params_str
                                            ))
                                        }
                                        Err(e) => Some(format!(
                                            "Error parsing Client.OnDisconnect: {}",
                                            e
                                        )),
                                    }
                                }
                                "Client.OnVolumeChanged" => {
                                    match serde_json::from_str::<ClientOnVolumeChanged>(&msg) {
                                        Ok(n) => {
                                            let params_str =
                                                serde_json::to_string(&n.params).unwrap_or_default();
                                            Some(format!(
                                                "{} - {}\n{}",
                                                formatted_time, n.method, params_str
                                            ))
                                        }
                                        Err(e) => Some(format!(
                                            "Error parsing Client.OnVolumeChanged: {}",
                                            e
                                        )),
                                    }
                                }
                                "Client.OnLatencyChanged" => {
                                    match serde_json::from_str::<ClientOnLatencyChanged>(&msg) {
                                        Ok(n) => {
                                            let params_str =
                                                serde_json::to_string(&n.params).unwrap_or_default();
                                            Some(format!(
                                                "{} - {}\n{}",
                                                formatted_time, n.method, params_str
                                            ))
                                        }
                                        Err(e) => Some(format!(
                                            "Error parsing Client.OnLatencyChanged: {}",
                                            e
                                        )),
                                    }
                                }
                                "Client.OnNameChanged" => {
                                    match serde_json::from_str::<ClientOnNameChanged>(&msg) {
                                        Ok(n) => {
                                            let params_str =
                                                serde_json::to_string(&n.params).unwrap_or_default();
                                            Some(format!(
                                                "{} - {}\n{}",
                                                formatted_time, n.method, params_str
                                            ))
                                        }
                                        Err(e) => Some(format!(
                                            "Error parsing Client.OnNameChanged: {}",
                                            e
                                        )),
                                    }
                                }
                                "Group.OnMute" => {
                                    match serde_json::from_str::<GroupOnMute>(&msg) {
                                        Ok(n) => {
                                            let params_str =
                                                serde_json::to_string(&n.params).unwrap_or_default();
                                            Some(format!(
                                                "{} - {}\n{}",
                                                formatted_time, n.method, params_str
                                            ))
                                        }
                                        Err(e) => {
                                            Some(format!("Error parsing Group.OnMute: {}", e))
                                        }
                                    }
                                }
                                "Group.OnStreamChanged" => {
                                    match serde_json::from_str::<GroupOnStreamChanged>(&msg) {
                                        Ok(n) => {
                                            let params_str =
                                                serde_json::to_string(&n.params).unwrap_or_default();
                                            Some(format!(
                                                "{} - {}\n{}",
                                                formatted_time, n.method, params_str
                                            ))
                                        }
                                        Err(e) => Some(format!(
                                            "Error parsing Group.OnStreamChanged: {}",
                                            e
                                        )),
                                    }
                                }
                                "Group.OnNameChanged" => {
                                    match serde_json::from_str::<GroupOnNameChanged>(&msg) {
                                        Ok(n) => {
                                            let params_str =
                                                serde_json::to_string(&n.params).unwrap_or_default();
                                            Some(format!(
                                                "{} - {}\n{}",
                                                formatted_time, n.method, params_str
                                            ))
                                        }
                                        Err(e) => Some(format!(
                                            "Error parsing Group.OnNameChanged: {}",
                                            e
                                        )),
                                    }
                                }
                                "Stream.OnUpdate" => {
                                    match serde_json::from_str::<StreamOnUpdate>(&msg) {
                                        Ok(n) => {
                                            let params_str =
                                                serde_json::to_string(&n.params).unwrap_or_default();
                                            Some(format!(
                                                "{} - {}\n{}",
                                                formatted_time, n.method, params_str
                                            ))
                                        }
                                        Err(e) => {
                                            Some(format!("Error parsing Stream.OnUpdate: {}", e))
                                        }
                                    }
                                }
                                "Stream.OnProperties" => {
                                    match serde_json::from_str::<StreamOnProperties>(&msg) {
                                        Ok(n) => {
                                            let params_str =
                                                serde_json::to_string(&n.params).unwrap_or_default();
                                            Some(format!(
                                                "{} - {}\n{}",
                                                formatted_time, n.method, params_str
                                            ))
                                        }
                                        Err(e) => Some(format!(
                                            "Error parsing Stream.OnProperties: {}",
                                            e
                                        )),
                                    }
                                }
                                "Server.OnUpdate" => {
                                    match serde_json::from_str::<ServerOnUpdate>(&msg) {
                                        Ok(n) => {
                                            let params_str =
                                                serde_json::to_string(&n.params).unwrap_or_default();
                                            Some(format!(
                                                "{} - {}\n{}",
                                                formatted_time, n.method, params_str
                                            ))
                                        }
                                        Err(e) => {
                                            Some(format!("Error parsing Server.OnUpdate: {}", e))
                                        }
                                    }
                                }
                                _ => None,
                            };

                            if let Some(event) = event_string {
                                events.push(event);
                            }
                        }
                    }
                }
            }
        });

        // Helper function to send a command and store its method
        let send_command = |request: String, app_state: &AppState, cmd_tx: &mpsc::Sender<String>| {
            if let Ok(json_value) = serde_json::from_str::<Value>(&request) {
                if let (Some(id_val), Some(method_val)) =
                    (json_value.get("id"), json_value.get("method"))
                {
                    if let (Some(id), Some(method)) = (id_val.as_str(), method_val.as_str()) {
                        let mut methods = app_state.request_methods.lock().unwrap();
                        methods.insert(id.to_string(), method.to_string());
                    }
                }
            }
            if let Err(_) = cmd_tx.try_send(request) {}
        };

        loop {
            // Handle cursor blinking
            {
                let mut last_toggle = self.app_state.last_cursor_toggle.lock().unwrap();
                if last_toggle.elapsed() > Duration::from_millis(500) {
                    let mut visible = self.app_state.cursor_visible.lock().unwrap();
                    *visible = !*visible;
                    *last_toggle = Instant::now();
                }
            }

            if let Err(_) = draw_ui(&mut self.terminal, &self.app_state) {
                break;
            }

            // This block ensures all guards are dropped at the end of the scope
            {
                let mut active_tab_guard = self.app_state.active_tab.lock().unwrap();
                let mut selected_index_guard = self.app_state.selected_index.lock().unwrap();
                let mut focused_panel_guard = self.app_state.focused_panel.lock().unwrap();
                let mut group_focused_field_guard =
                    self.app_state.group_focused_field.lock().unwrap();
                let mut client_focused_field_guard =
                    self.app_state.client_focused_field.lock().unwrap();
                let mut is_editing_group_name_guard =
                    self.app_state.is_editing_group_name.lock().unwrap();
                let mut is_editing_name_guard =
                    self.app_state.is_editing_client_name.lock().unwrap();
                let mut is_editing_volume_guard =
                    self.app_state.is_editing_client_volume.lock().unwrap();
                let mut is_editing_group_stream_guard =
                    self.app_state.is_editing_group_stream.lock().unwrap();
                let mut is_editing_group_muted_guard =
                    self.app_state.is_editing_group_muted.lock().unwrap();
                let mut is_editing_client_muted_guard =
                    self.app_state.is_editing_client_muted.lock().unwrap();
                let mut is_editing_group_clients_guard =
                    self.app_state.is_editing_group_clients.lock().unwrap();
                let mut is_editing_latency_guard =
                    self.app_state.is_editing_client_latency.lock().unwrap();
                let status_data_guard = self.status_data.lock().unwrap();

                let max_items = if let Some(data) = &*status_data_guard {
                    match *active_tab_guard {
                        TabSelection::Groups => data.result.server.groups.len(),
                        TabSelection::Clients => data
                            .result
                            .server
                            .groups
                            .iter()
                            .map(|g| g.clients.len())
                            .sum(),
                        TabSelection::Streams => data.result.server.streams.len(),
                    }
                } else {
                    0
                };

                let input_event = handle_input(
                    &mut active_tab_guard,
                    &mut selected_index_guard,
                    max_items,
                    &mut focused_panel_guard,
                    *is_editing_group_stream_guard,
                    *is_editing_group_muted_guard,
                    *is_editing_client_muted_guard,
                    *is_editing_group_clients_guard,
                    *is_editing_group_name_guard,
                    *is_editing_name_guard,
                    *is_editing_volume_guard,
                    *is_editing_latency_guard,
                    &mut group_focused_field_guard,
                    &mut client_focused_field_guard,
                );

                let client_focused_field = client_focused_field_guard.clone();

                match input_event {
                    Ok(InputEvent::Quit) => break,
                    Ok(InputEvent::ToggleFocus) => {}
                    Ok(InputEvent::ReverseToggleFocus) => {}
                    Ok(InputEvent::Left) => {
                        if *focused_panel_guard == PanelFocus::Events {
                            let mut offset = self.app_state.events_scroll_offset.lock().unwrap();
                            *offset = offset.saturating_sub(1);
                        }
                    }
                    Ok(InputEvent::Right) => {
                        if *focused_panel_guard == PanelFocus::Events {
                            let mut offset = self.app_state.events_scroll_offset.lock().unwrap();
                            *offset += 1;
                        }
                    }
                    Ok(InputEvent::Up) => {
                        if *focused_panel_guard == PanelFocus::List {
                            if *selected_index_guard > 0 {
                                *selected_index_guard -= 1;
                            }
                        } else if *is_editing_group_stream_guard {
                            let mut stream_idx =
                                self.app_state.stream_selection_index.lock().unwrap();
                            if *stream_idx > 0 {
                                *stream_idx -= 1;
                            }
                        } else if *is_editing_group_muted_guard {
                            let mut selection_idx =
                                self.app_state.group_muted_selection_index.lock().unwrap();
                            if *selection_idx > 0 {
                                *selection_idx -= 1;
                            }
                        } else if *is_editing_client_muted_guard {
                            let mut selection_idx =
                                self.app_state.client_muted_selection_index.lock().unwrap();
                            if *selection_idx > 0 {
                                *selection_idx -= 1;
                            }
                        } else if *is_editing_group_clients_guard {
                            let mut client_idx =
                                self.app_state.client_selection_index.lock().unwrap();
                            if *client_idx > 0 {
                                *client_idx -= 1;
                            }
                        } else {
                            match *active_tab_guard {
                                TabSelection::Groups => {
                                    *group_focused_field_guard =
                                        get_previous_group_field(&group_focused_field_guard)
                                }
                                TabSelection::Clients => {
                                    *client_focused_field_guard =
                                        get_previous_client_field(&client_focused_field_guard)
                                }
                                _ => {}
                            }
                        }
                    }
                    Ok(InputEvent::Down) => {
                        if *focused_panel_guard == PanelFocus::List {
                            if *selected_index_guard < max_items.saturating_sub(1) {
                                *selected_index_guard += 1;
                            }
                        } else if *is_editing_group_stream_guard {
                            let streams_len = if let Some(data) = &*status_data_guard {
                                data.result.server.streams.len()
                            } else {
                                0
                            };
                            let mut stream_idx =
                                self.app_state.stream_selection_index.lock().unwrap();
                            if *stream_idx < streams_len.saturating_sub(1) {
                                *stream_idx += 1;
                            }
                        } else if *is_editing_group_muted_guard {
                            let mut selection_idx =
                                self.app_state.group_muted_selection_index.lock().unwrap();
                            if *selection_idx < 1 {
                                *selection_idx += 1;
                            }
                        } else if *is_editing_client_muted_guard {
                            let mut selection_idx =
                                self.app_state.client_muted_selection_index.lock().unwrap();
                            if *selection_idx < 1 {
                                *selection_idx += 1;
                            }
                        } else if *is_editing_group_clients_guard {
                            let clients_len = if let Some(data) = &*status_data_guard {
                                data.result
                                    .server
                                    .groups
                                    .iter()
                                    .flat_map(|g| &g.clients)
                                    .count()
                            } else {
                                0
                            };
                            let mut client_idx =
                                self.app_state.client_selection_index.lock().unwrap();
                            if *client_idx < clients_len.saturating_sub(1) {
                                *client_idx += 1;
                            }
                        } else {
                            match *active_tab_guard {
                                TabSelection::Groups => {
                                    *group_focused_field_guard =
                                        get_next_group_field(&group_focused_field_guard)
                                }
                                TabSelection::Clients => {
                                    *client_focused_field_guard =
                                        get_next_client_field(&client_focused_field_guard)
                                }
                                _ => {}
                            }
                        }
                    }
                    Ok(InputEvent::Edit) => {
                        if *focused_panel_guard == PanelFocus::Details {
                            match *active_tab_guard {
                                TabSelection::Groups => match *group_focused_field_guard {
                                    GroupDetailsFocus::Name => {
                                        if !*is_editing_group_name_guard {
                                            *is_editing_group_name_guard = true;
                                            let mut editing_name = self
                                                .app_state
                                                .editing_group_name
                                                .lock()
                                                .unwrap();
                                            if let Some(data) = &*status_data_guard {
                                                if let Some(group) = data
                                                    .result
                                                    .server
                                                    .groups
                                                    .get(*selected_index_guard)
                                                {
                                                    *editing_name = group.name.clone();
                                                }
                                            }
                                        }
                                    }
                                    GroupDetailsFocus::StreamId => {
                                        *is_editing_group_stream_guard = true;
                                        *self.app_state.stream_selection_index.lock().unwrap() = 0;
                                    }
                                    GroupDetailsFocus::Muted => {
                                        *is_editing_group_muted_guard = true;
                                        let mut selection_idx = self
                                            .app_state
                                            .group_muted_selection_index
                                            .lock()
                                            .unwrap();
                                        if let Some(data) = &*status_data_guard {
                                            if let Some(group) = data
                                                .result
                                                .server
                                                .groups
                                                .get(*selected_index_guard)
                                            {
                                                *selection_idx = if group.muted { 0 } else { 1 };
                                            }
                                        }
                                    }
                                    GroupDetailsFocus::Clients => {
                                        *is_editing_group_clients_guard = true;
                                        let mut selected_clients =
                                            self.app_state.selected_clients.lock().unwrap();
                                        if let Some(data) = &*status_data_guard {
                                            if let Some(group) = data
                                                .result
                                                .server
                                                .groups
                                                .get(*selected_index_guard)
                                            {
                                                *selected_clients = group
                                                    .clients
                                                    .iter()
                                                    .map(|c| c.id.clone())
                                                    .collect();
                                            }
                                        }
                                    }
                                    _ => {}
                                },
                                TabSelection::Clients => match client_focused_field {
                                    ClientDetailsFocus::Name => {
                                        if !*is_editing_name_guard {
                                            *is_editing_name_guard = true;
                                            let mut editing_name = self
                                                .app_state
                                                .editing_client_name
                                                .lock()
                                                .unwrap();
                                            if let Some(data) = &*status_data_guard {
                                                let mut client_count = 0;
                                                'outer: for group in &data.result.server.groups {
                                                    for client in &group.clients {
                                                        if client_count == *selected_index_guard {
                                                            *editing_name =
                                                                client.config.name.clone();
                                                            break 'outer;
                                                        }
                                                        client_count += 1;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    ClientDetailsFocus::Volume => {
                                        if !*is_editing_volume_guard {
                                            *is_editing_volume_guard = true;
                                            let mut editing_volume = self
                                                .app_state
                                                .editing_client_volume
                                                .lock()
                                                .unwrap();
                                            if let Some(data) = &*status_data_guard {
                                                let mut client_count = 0;
                                                'outer: for group in &data.result.server.groups {
                                                    for client in &group.clients {
                                                        if client_count == *selected_index_guard {
                                                            *editing_volume = client
                                                                .config
                                                                .volume
                                                                .percent
                                                                .to_string();
                                                            break 'outer;
                                                        }
                                                        client_count += 1;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    ClientDetailsFocus::Muted => {
                                        *is_editing_client_muted_guard = true;
                                        let mut selection_idx = self
                                            .app_state
                                            .client_muted_selection_index
                                            .lock()
                                            .unwrap();
                                        if let Some(data) = &*status_data_guard {
                                            let mut client_count = 0;
                                            'outer: for group in &data.result.server.groups {
                                                for client in &group.clients {
                                                    if client_count == *selected_index_guard {
                                                        *selection_idx =
                                                            if client.config.volume.muted {
                                                                0
                                                            } else {
                                                                1
                                                            };
                                                        break 'outer;
                                                    }
                                                    client_count += 1;
                                                }
                                            }
                                        }
                                    }
                                    ClientDetailsFocus::Latency => {
                                        if !*is_editing_latency_guard {
                                            *is_editing_latency_guard = true;
                                            let mut editing_latency = self
                                                .app_state
                                                .editing_client_latency
                                                .lock()
                                                .unwrap();
                                            if let Some(data) = &*status_data_guard {
                                                let mut client_count = 0;
                                                'outer: for group in &data.result.server.groups {
                                                    for client in &group.clients {
                                                        if client_count == *selected_index_guard {
                                                            *editing_latency = client
                                                                .config
                                                                .latency
                                                                .to_string();
                                                            break 'outer;
                                                        }
                                                        client_count += 1;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                },
                                _ => {}
                            }
                        }
                    }
                    Ok(InputEvent::ToggleSelection) => {
                        if *is_editing_group_clients_guard {
                            if let Some(data) = &*status_data_guard {
                                let mut selected_clients =
                                    self.app_state.selected_clients.lock().unwrap();
                                let client_idx =
                                    *self.app_state.client_selection_index.lock().unwrap();
                                let all_clients: Vec<_> = data
                                    .result
                                    .server
                                    .groups
                                    .iter()
                                    .flat_map(|g| &g.clients)
                                    .collect();
                                if let Some(client) = all_clients.get(client_idx) {
                                    if selected_clients.contains(&client.id) {
                                        selected_clients.retain(|id| id != &client.id);
                                    } else {
                                        selected_clients.push(client.id.clone());
                                    }
                                }
                            }
                        }
                    }
                    Ok(InputEvent::Confirm) => {
                        if *is_editing_group_name_guard {
                            *is_editing_group_name_guard = false;
                            if let Some(data) = &*status_data_guard {
                                if let Some(group) =
                                    data.result.server.groups.get(*selected_index_guard)
                                {
                                    let group_id = group.id.clone();
                                    let new_name =
                                        self.app_state.editing_group_name.lock().unwrap().clone();
                                    let set_name_request =
                                        crate::commands::group::setname::create_set_name_request(
                                            &group_id, &new_name,
                                        );
                                    send_command(set_name_request, &self.app_state, &self.cmd_tx);
                                }
                            }
                        } else if *is_editing_group_stream_guard {
                            *is_editing_group_stream_guard = false;
                            if let Some(data) = &*status_data_guard {
                                if let Some(group) =
                                    data.result.server.groups.get(*selected_index_guard)
                                {
                                    let stream_idx =
                                        *self.app_state.stream_selection_index.lock().unwrap();
                                    if let Some(stream) =
                                        data.result.server.streams.get(stream_idx)
                                    {
                                        let set_stream_request = crate::commands::group::setstream::create_set_stream_request(&group.id, &stream.id);
                                        send_command(
                                            set_stream_request,
                                            &self.app_state,
                                            &self.cmd_tx,
                                        );
                                    }
                                }
                            }
                        } else if *is_editing_group_muted_guard {
                            *is_editing_group_muted_guard = false;
                            let selection_idx =
                                *self.app_state.group_muted_selection_index.lock().unwrap();
                            let new_mute_status = selection_idx == 0;
                            if let Some(data) = &*status_data_guard {
                                if let Some(group) =
                                    data.result.server.groups.get(*selected_index_guard)
                                {
                                    let set_mute_request =
                                        crate::commands::group::setmute::create_set_mute_request(
                                            &group.id,
                                            new_mute_status,
                                        );
                                    send_command(
                                        set_mute_request,
                                        &self.app_state,
                                        &self.cmd_tx,
                                    );
                                }
                            }
                        } else if *is_editing_client_muted_guard {
                            *is_editing_client_muted_guard = false;
                            let selection_idx =
                                *self.app_state.client_muted_selection_index.lock().unwrap();
                            let new_muted_status = selection_idx == 0;
                            if let Some(data) = &*status_data_guard {
                                let mut client_count = 0;
                                'outer: for group in &data.result.server.groups {
                                    for client in &group.clients {
                                        if client_count == *selected_index_guard {
                                            let set_volume_request = crate::commands::client::setvolume::create_set_volume_request(
                                                &client.id,
                                                new_muted_status,
                                                client.config.volume.percent,
                                            );
                                            send_command(
                                                set_volume_request,
                                                &self.app_state,
                                                &self.cmd_tx,
                                            );
                                            break 'outer;
                                        }
                                        client_count += 1;
                                    }
                                }
                            }
                        } else if *is_editing_group_clients_guard {
                            *is_editing_group_clients_guard = false;
                            if let Some(data) = &*status_data_guard {
                                if let Some(group) =
                                    data.result.server.groups.get(*selected_index_guard)
                                {
                                    let selected_clients =
                                        self.app_state.selected_clients.lock().unwrap().clone();
                                    let set_clients_request = crate::commands::group::setclients::create_set_clients_request(&group.id, selected_clients);
                                    send_command(
                                        set_clients_request,
                                        &self.app_state,
                                        &self.cmd_tx,
                                    );
                                }
                            }
                        } else if *is_editing_name_guard {
                            *is_editing_name_guard = false;
                            if let Some(data) = &*status_data_guard {
                                let mut client_count = 0;
                                'outer: for group in &data.result.server.groups {
                                    for client in &group.clients {
                                        if client_count == *selected_index_guard {
                                            let client_id = client.id.clone();
                                            let new_name = self
                                                .app_state
                                                .editing_client_name
                                                .lock()
                                                .unwrap()
                                                .clone();
                                            let set_name_request =
                                                crate::commands::client::setname::create_set_name_request(
                                                    &client_id, &new_name,
                                                );
                                            send_command(
                                                set_name_request,
                                                &self.app_state,
                                                &self.cmd_tx,
                                            );
                                            break 'outer;
                                        }
                                        client_count += 1;
                                    }
                                }
                            }
                        } else if *is_editing_volume_guard {
                            *is_editing_volume_guard = false;
                            let new_volume_str =
                                self.app_state.editing_client_volume.lock().unwrap().clone();
                            if let Ok(new_volume) = new_volume_str.parse::<u32>() {
                                if (0..=100).contains(&new_volume) {
                                    if let Some(data) = &*status_data_guard {
                                        let mut client_count = 0;
                                        'outer: for group in &data.result.server.groups {
                                            for client in &group.clients {
                                                if client_count == *selected_index_guard {
                                                    let client_id = client.id.clone();
                                                    let set_volume_request = crate::commands::client::setvolume::create_set_volume_request(
                                                        &client_id,
                                                        client.config.volume.muted,
                                                        new_volume,
                                                    );
                                                    send_command(
                                                        set_volume_request,
                                                        &self.app_state,
                                                        &self.cmd_tx,
                                                    );
                                                    break 'outer;
                                                }
                                                client_count += 1;
                                            }
                                        }
                                    }
                                } else {
                                }
                            }
                        } else if *is_editing_latency_guard {
                            *is_editing_latency_guard = false;
                            let new_latency_str = self
                                .app_state
                                .editing_client_latency
                                .lock()
                                .unwrap()
                                .clone();
                            if let Ok(new_latency) = new_latency_str.parse::<i32>() {
                                if let Some(data) = &*status_data_guard {
                                    let mut client_count = 0;
                                    'outer: for group in &data.result.server.groups {
                                        for client in &group.clients {
                                            if client_count == *selected_index_guard {
                                                let client_id = client.id.clone();
                                                let set_latency_request = crate::commands::client::setlatency::create_set_latency_request(
                                                    &client_id,
                                                    new_latency,
                                                );
                                                send_command(
                                                    set_latency_request,
                                                    &self.app_state,
                                                    &self.cmd_tx,
                                                );
                                                break 'outer;
                                            }
                                            client_count += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Ok(InputEvent::Cancel) => {
                        if *is_editing_group_name_guard {
                            *is_editing_group_name_guard = false;
                        }
                        if *is_editing_group_stream_guard {
                            *is_editing_group_stream_guard = false;
                        }
                        if *is_editing_group_muted_guard {
                            *is_editing_group_muted_guard = false;
                        }
                        if *is_editing_client_muted_guard {
                            *is_editing_client_muted_guard = false;
                        }
                        if *is_editing_group_clients_guard {
                            *is_editing_group_clients_guard = false;
                        }
                        if *is_editing_name_guard {
                            *is_editing_name_guard = false;
                        }
                        if *is_editing_volume_guard {
                            *is_editing_volume_guard = false;
                        }
                        if *is_editing_latency_guard {
                            *is_editing_latency_guard = false;
                        }
                    }
                    Ok(InputEvent::Char(c)) => {
                        if *is_editing_group_name_guard {
                            self.app_state.editing_group_name.lock().unwrap().push(c);
                        } else if *is_editing_name_guard {
                            self.app_state
                                .editing_client_name
                                .lock()
                                .unwrap()
                                .push(c);
                        } else if *is_editing_volume_guard {
                            if c.is_ascii_digit() {
                                self.app_state
                                    .editing_client_volume
                                    .lock()
                                    .unwrap()
                                    .push(c);
                            }
                        } else if *is_editing_latency_guard {
                            if c.is_ascii_digit() {
                                self.app_state
                                    .editing_client_latency
                                    .lock()
                                    .unwrap()
                                    .push(c);
                            }
                        }
                    }
                    Ok(InputEvent::Backspace) => {
                        if *is_editing_group_name_guard {
                            self.app_state.editing_group_name.lock().unwrap().pop();
                        } else if *is_editing_name_guard {
                            self.app_state.editing_client_name.lock().unwrap().pop();
                        } else if *is_editing_volume_guard {
                            self.app_state
                                .editing_client_volume
                                .lock()
                                .unwrap()
                                .pop();
                        } else if *is_editing_latency_guard {
                            self.app_state
                                .editing_client_latency
                                .lock()
                                .unwrap()
                                .pop();
                        }
                    }
                    Ok(InputEvent::TabChanged(_)) => {
                        if *is_editing_group_name_guard {
                            *is_editing_group_name_guard = false;
                        }
                        if *is_editing_group_stream_guard {
                            *is_editing_group_stream_guard = false;
                        }
                        if *is_editing_group_muted_guard {
                            *is_editing_group_muted_guard = false;
                        }
                        if *is_editing_client_muted_guard {
                            *is_editing_client_muted_guard = false;
                        }
                        if *is_editing_group_clients_guard {
                            *is_editing_group_clients_guard = false;
                        }
                        if *is_editing_name_guard {
                            *is_editing_name_guard = false;
                        }
                        if *is_editing_volume_guard {
                            *is_editing_volume_guard = false;
                        }
                        if *is_editing_latency_guard {
                            *is_editing_latency_guard = false;
                        }
                        *focused_panel_guard = PanelFocus::List;
                    }
                    Ok(_) => {}
                    Err(_) => {
                        break;
                    }
                }
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        restore_terminal()
    }
}