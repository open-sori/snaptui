# snaptui

A terminal user interface (TUI) for the multi-room audio player [Snapcast](https://github.com/badaix/snapcast), built with Rust.

![screenshot](https://raw.githubusercontent.com/open-sori/snaptui/main/screenshot.png)

## Features

- **Real-time Monitoring**: Connects to your Snapcast server and displays live information.
- **Tabbed Interface**: Easily switch between views for Groups, Clients, and Streams.
- **Interactive Control**:
  - **Groups**:
    - Rename groups.
    - Change the assigned audio stream.
    - Mute/unmute groups.
    - Assign clients to groups.
  - **Clients**:
    - Rename clients.
    - Adjust client volume.
    - Mute/unmute clients.
    - Set client latency.
- **Responsive UI**: Built with `ratatui` for a smooth terminal experience.
- **Lightweight**: Runs as a single binary.

## Installation

### From Crates.io

```bash
cargo install snaptui
```

### From Source

1. Clone the repository:

    ```bash
    git clone https://github.com/open-sori/snaptui.git
    cd snaptui
    ```

2. Build the release binary:

    ```bash
    cargo build --release
    ```

3. The binary will be located at `target/release/snaptui`.

## Usage

Run the application with:

```bash
snaptui [OPTIONS]
```

### Command-line Arguments

| Option      | Environment Variable  | Default     | Description                       |
|-------------|-----------------------|-------------|-----------------------------------|
| `--host`    | `SNAPSERVER_HOST`     | `127.0.0.1` | The IP address of the Snapcast server. |
| `--port`    | `SNAPSERVER_PORT`     | `1780`      | The JSON-RPC port of the Snapcast server. |
| `--version` |                       |             | Print version information.        |
| `-h`, `--help`|                     |             | Print help information.           |

### Example

```bash
# Connect to a server on a different host
snaptui --host 192.168.1.100

# Connect to a server with a non-default port
snaptui --host 192.168.1.100 --port 1788
```

## Keybindings

| Key(s)    | Action                                       |
|-----------|----------------------------------------------|
| `q`       | Quit the application.                        |
| `←` / `→` | Switch between the main tabs (Groups, Clients, Streams). |
| `↑` / `↓` | Navigate up/down in lists or detail fields.  |
| `Tab`     | Toggle focus between the list and details panels. |
| `e`       | Enter "edit mode" for the selected field.    |
| `Enter`   | Confirm the change in "edit mode".           |
| `Esc`     | Cancel "edit mode".                          |
| `Space`   | Toggle selection (e.g., for group clients).  |

## Building the Container Image

### Manualy build the image

```bash
docker build --build-arg CREATED_DATE="$(date +'%Y-%m-%d')" --build-arg SNAPTUI_VERSION="v1.0.0" --build-arg TARGETARCH="arm64" -t snaptui .
docker run --rm --interactive --tty snaptui
export GHCR_PAT="YOUR_TOKEN"
echo $GHCR_PAT | docker login ghcr.io -u tdesaules --password-stdin
docker image tag snaptui:latest ghcr.io/open-sori/snaptui:v1.0.0
docker push ghcr.io/open-sori/snaptui:v1.0.0
docker pull ghcr.io/open-sori/snaptui:v1.0.0
```

## Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/open-sori/snaptui/issues).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.