# Deployment

AEQI is best deployed as one product with a thin edge proxy.

## Recommended Topology

- `aeqi` daemon handles orchestration, workers, background logic, and persistence
- `aeqi-web` handles the HTTP API and can also serve the compiled UI
- `nginx` or `caddy` sits in front only for TLS termination, host routing, and standard reverse-proxy concerns

## Why This Model

- The open-source install stays simple: one backend application surface instead of split frontend hosting requirements
- The UI and API ship together, so version drift is reduced
- Standard edge tooling still handles TLS, compression, and host-based routing cleanly

## Local Development

Use separate processes:

- `aeqi` daemon
- `aeqi-web`
- `apps/ui` Vite server

That gives you fast frontend iteration without changing the production shape.

## Production Build

1. Build the UI with `npm run ui:build`
2. Point `[web].ui_dist_dir` at `../apps/ui/dist` or an absolute path
3. Run `aeqi web start`
4. Put `nginx` or `caddy` in front of it

## systemd

The intended service model is:

- `aeqi.service` for the daemon
- `aeqi-web.service` for the API and UI surface

The reverse proxy should treat `aeqi-web` as the single upstream for both `/api` and browser routes.
