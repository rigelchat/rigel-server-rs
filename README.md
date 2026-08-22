<div align="center">
  <img src="https://avatars.githubusercontent.com/u/247460033?s=200&v=4" alt="Rigel Server" width="100" height="100"/>

  # Rigel Core Server

  An alternative implementation of the Discord API, written in Rust.

  [![Release](https://img.shields.io/github/v/release/rigelchat/server?style=for-the-badge&logo=github)](https://github.com/rigelchat/server/releases/latest)
  [![License](https://img.shields.io/badge/License-AGPLv3-blue.svg?style=for-the-badge)](LICENSE)
  [![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
</div>

## Introduction

Rigel is a self-hostable, alternative server implementation compatible with the Discord API and client ecosystem. Developed as an independent project, it is designed to be compatible with both [**Spacebar**](https://github.com/spacebarchat) or [**Fluxer**](https://github.com/fluxerapp/fluxer).

This repository contains the core backend codebase, including the REST API, Gateway (WebSocket), and CDN.

## Prerequisites

* **MySQL** (>= 8.0) or **MariaDB** (>= 10.5)
* *(Optional)* **Rust toolchain** (latest stable, only if building from source)

---

## Installation & Setup

### 1. Database Initialization

Create a dedicated database for Rigel with utf8mb4 encoding:

```sql
CREATE DATABASE rigel CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
```

---

### Option A: Using Pre-built Binaries (Recommended)

1. Download the archive matching your OS and architecture from the [**Latest Releases**](https://github.com/rigelchat/server/releases/latest).
2. Extract the archive into your desired directory.
3. Copy `.env.example` to `.env` (or create one) and adjust your configuration.
   ```bash
   cp .env.example .env
   ```
4. Run the executable (database migrations will run automatically on startup):
   ```bash
   # Linux / macOS
   chmod +x rigel-server
   ./rigel-server

   # Windows
   .\rigel-server.exe
   ```

> [!IMPORTANT]
> Most variables have built-in default values. At minimum, ensure your `DATABASE_URL` is configured.

---

### Option B: Building from Source (Developers)

1. Clone the repository:
   ```bash
   git clone https://github.com/rigelchat/server.git
   cd server
   ```

2. Configure environment variables:
   ```bash
   cp .env.example .env
   # Edit .env with your database credentials and secret key
   ```

3. Run in development mode (with auto-migrations):
   ```bash
   cargo run
   ```

4. Or build an optimized release binary:
   ```bash
   cargo build --release
   ./target/release/rigel-server
   ```

---

## Server Endpoints

Once started, the server listens by default on port `3000`:
* **REST API:** `http://localhost:3000/api/v1` (alias: `http://localhost:3000/api`)
* **Gateway (WebSocket):** `ws://localhost:3000/gateway`

## Testing

Run the automated test suite:

```bash
cargo test
```

## API Documentation

Rigel aims for 1:1 compatibility with the Discord API. Refer to the official [Discord Developer Portal](https://discord.com/developers/docs/intro) or check our [Wiki](https://github.com/rigelchat/server/wiki) for project-specific details.

---

<div align="center">
  <sub>Part of the <a href="https://github.com/rigelchat">Rigel Project</a>.</sub>
</div>