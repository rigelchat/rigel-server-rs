<div align="center">
  <img src="https://avatars.githubusercontent.com/u/247460033?s=200&v=4" alt="Rigel Server" width="100" height="100"/>

  # Rigel Core Server

  **The robust Rust backend powering the Rigel ecosystem.**

  Provides a Discord-compatible REST API, Gateway and CDN.

  [![Rigel](https://img.shields.io/badge/Rigel-Join_Public_Instance-brightgreen?style=for-the-badge&logo=rocket&logoColor=white)](https://app.rigel.chat/invite/rigel?instance=https%3A%2F%2Fserver.rigel.chat)
  [![License](https://img.shields.io/badge/License-AGPLv3-blue.svg?style=for-the-badge)](LICENSE)
  [![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
</div>

## ✨ Introduction

This repository houses the API, Gateway (WebSocket), and CDN for Rigel. It is designed to be self-hostable, highly performant, and compatible with the existing Discord ecosystem of bots and tools.

Rigel Core is also fully compatible with the **[Spacebar](https://github.com/spacebarchat)** project, allowing a seamless transition for users and developers coming from that ecosystem.

## 🛠️ Prerequisites

* [Rust](https://rust-lang.org/tools/install) (latest stable recommended)
* [MySQL](https://dev.mysql.com/downloads) (A MySQL/MariaDB server is required to store data)

## 🚀 Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/rigelchat/rigel-server-rs.git
cd rigel-server-rs
```

### 2. Database Setup

Rigel requires a MySQL database to run. You can set it up natively or via Docker.

Install MySQL:
```bash
sudo apt install mariadb # (recommended)
# or
sudo apt install mysql-server

```

Log into your MySQL console:
```bash
mysql -u root
```

Then, create the database for Rigel:
```sql
CREATE DATABASE rigel CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
EXIT;
```

### 3. Configuration

Copy the example environment file and configure your database and keys:

```bash
cp .env.example .env
```

*Edit `.env` to set your `PORT`, `DATABASE_URL`, and `AUTH_SECRET`.*

### 4. Run Development Server

To build and run the server using Cargo. This will automatically run any pending database migrations.

```bash
cargo run
```

### 5. Build for Production

```bash
cargo build --release
```

The API will be available at `http://localhost:3000/api/v0`.

## 🧪 Testing

To run the test suite:

```bash
cargo test
```

## 📚 API Documentation

Since Rigel aims for **Discord API Compatibility**, you can refer to the official Discord documentation for endpoints structure, or check our local documentation in the `/docs` folder.

---

<div align="center">
  <sub>Part of the <a href="https://github.com/rigelchat">Rigel Project</a>.</sub>
</div>