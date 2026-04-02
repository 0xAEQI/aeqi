# Quick Start

This guide gets AEQI running locally with the daemon, API, and web UI.

## Prerequisites

- Rust stable
- Node.js 22+
- At least one model provider key

## 1. Create Local Config

```bash
cp config/aeqi.example.toml config/aeqi.toml
```

`config/aeqi.toml` is local-only and should stay uncommitted.

Configure a provider and enable the web server:

```toml
[providers.openrouter]
api_key = "${OPENROUTER_API_KEY}"
default_model = "xiaomi/mimo-v2-pro"

[web]
enabled = true
bind = "127.0.0.1:8400"
ui_dist_dir = "../apps/ui/dist"
auth_secret = "${AEQI_WEB_SECRET}"
```

## 2. Build

```bash
cargo build
npm run ui:install
npm run ui:build
```

## 3. Start AEQI

In one shell:

```bash
cargo run --bin aeqi -- daemon start
```

In a second shell:

```bash
export AEQI_WEB_SECRET=change-me
cargo run --bin aeqi -- web start
```

Open `http://127.0.0.1:8400`.

## 4. UI Development Mode

If you want Vite hot reload instead of the compiled UI:

```bash
npm run ui:dev
```

That serves the frontend on `http://127.0.0.1:5173` and proxies `/api/*` to `aeqi-web` on `:8400`.
