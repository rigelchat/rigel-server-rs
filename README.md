<div align="center">
  <img src="https://avatars.githubusercontent.com/u/247460033?s=200&v=4" alt="Rigel Server" width="100" height="100"/>

  # Rigel Core Server

  An alternative implementation of the Discord API, written in Rust.

  [![License](https://img.shields.io/badge/License-AGPLv3-blue.svg?style=for-the-badge)](LICENSE)
  [![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
</div>

## Introduction

Rigel is a self-hostable, alternative server implementation compatible with the Discord API and client ecosystem. Developed as an independent project, it is designed to be compatible with both **[Spacebar](https://github.com/spacebarchat)** or **[https://github.com/fluxerapp/fluxer](Fluxer)**.

This repository contains the backend codebase, which includes the REST API, Gateway (WebSocket), and CDN.

## Prerequisites

* Rust compiler (latest stable)
* MySQL or MariaDB

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/rigelchat/rigel-server-rs.git
cd rigel-server-rs
```

### 2. Database Setup

Rigel requires a MySQL or MariaDB database.

Log into your database console:
```sql
CREATE DATABASE rigel CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
```

### 3. Configuration

Copy the example environment file and configure your database connection and secrets:

```bash
cp .env.example .env
```

*Edit `.env` to set your own `PORT`, `DATABASE_URL`, and `AUTH_SECRET`.*

### 4. Running the Server

To start the server in development mode (this will automatically run database migrations):

```bash
cargo run
```

For production builds:

```bash
cargo build --release
```

The API will listen at `http://localhost:3000/api/v1` (also accessible via `http://localhost:3000/api`).

## Testing

Run the test suite with:

```bash
cargo test
```

## API Documentation

Rigel aims for compatibility with the Discord API. You can refer to the official Discord developer documentation or check the [Wiki](https://github.com/rigelchat/rigel-server-rs/wiki) for project-specific details.

---

<div align="center">
  <sub>Part of the <a href="https://github.com/rigelchat">Rigel Project</a>.<sub>
</div>