# Helios | Rust master backend

[![AGPL-3.0 License](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://choosealicense.com/licenses/agpl-3.0/)

Rust implementation of the Helios backend.

## Environment variables

| Variable | Description |
|----------|-------------|
| MASTER_BACKEND_PORT | Port for the backend |
| MASTER_BACKEND_HOST | Host for the backend |
| MASTER_BACKEND_URL | URL for the backend |
| MASTER_METRICS_PORT | Port for the metrics |
| MASTER_METRICS_HOST | Host for the metrics |
| MASTER_METRICS_URL | URL for the metrics |
| DATABASE_PORT | Port for the database |
| DATABASE_NAME | Name of the database |
| DATABASE_USER | User for the database |
| DATABASE_PASSWORD | Password for the database |
| DATABASE_HOST | Host for the database |
| DATABASE_URL | URL for the database |
| JWT_ACCESS_SECRET | Secret for the access token |
| JWT_REFRESH_SECRET | Secret for the refresh token |
| RUST_ENV | Environment for the backend |
| HELIOS_DOMAIN | Domain for Helios |
| DO_AUTH_TOKEN | Token for DigitalOcean |
| TRAEFIK_AUTH | Auth for Traefik |
| GRAFANA_USER | User for Grafana |
| GRAFANA_PASSWORD | Password for Grafana |

> github set secrets:

```bash
gh secret set -f .env
```
