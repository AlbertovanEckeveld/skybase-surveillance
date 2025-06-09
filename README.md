# Skybase Surveillance
Skybase Surveillance is a Rust-based application for automated RTSP camera health checking and minute-based video recording, storing the results in a configurable directory structure and using PostgreSQL for camera management.


## Features
- Loads configuration from config.toml
- Connects to a PostgreSQL database to fetch camera streams
- Checks RTSP camera health using ffmpeg
- Records 1-minute video segments for healthy cameras
- Stores recordings in a configurable directory structure
- Cross-platform: works on Windows and Linux

## Requirements
- Rust (latest stable)
- Cargo
- PostgreSQL
- ffmpeg (must be installed and in PATH)
- Docker (for cross-compilation with cross, optional)

## Configuration
Create a config.toml in the project root:
```toml
    [database]
    name = "<Name of your database>"
    user = "<Name of your database user>"
    password = "<password for your database user>"
    host = "<host of your database>"
    port = 5432
    
    [app]
    records_path = "./records"
```

### Database
The camera table must exist with at least the following columns:
- name (string)
- rtsp_url (string)
## Usage
### Build
Native (Windows or Linux)
```bash
  cargo build --release
```

### Cross-compile (optional)
```bash
  docker run --rm -v "$(pwd)":/app -w /app japaric/cross:latest cross build --release --target <target-triple>
```
### Run
```bash
  cargo run --release
```
### Run with Docker (optional)
```bash
  docker build -t skybase-surveillance .
  docker run --rm -v "$(pwd)/config.toml":/app/config.toml -v "$(pwd)/records":/app/records skybase-surveillance
```
### Run with Docker Compose (optional)
```bash
  docker-compose up --build
```

## Directory Structure
- src/config.rs: Configuration loading
- src/recorder.rs: Camera health and recording logic
- src/main.rs: Application entry point

MIT License. See LICENSE for details.