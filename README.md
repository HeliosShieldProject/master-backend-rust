# Helios | Master server

The Master Server is a robust and scalable Rust-based server designed to handle user requests, host a database, and perform various other functions essential for your application's backend operations. It communicates seamlessly with the Agent Server to ensure efficient data management and processing. This project leverages Docker for containerization, enabling easy deployment and management of services.

## Key Features

![image](readme/docker_master_server.png)

* **Traefik**: A versatile reverse proxy and load balancer for microservices. Traefik manages the routing of HTTP and HTTPS requests and provides SSL termination using Let's Encrypt.
* **Prometheus**: A powerful monitoring and alerting toolkit used to collect and store metrics from the Master Server and other services.
* **Grafana**: An open-source platform for monitoring and observability. Grafana integrates with Prometheus to visualize metrics and create detailed dashboards.
* **Loki**: A log aggregation system designed to store and query logs from various services.
* **Promtail**: An agent that ships logs from your Docker containers to Loki.
* **PostgreSQL**: A reliable and high-performance relational database system used to store application data.
* **Master Backend**: A Rust-based backend service that processes user requests and interacts with the PostgreSQL database.

## Flow

![image](readme/flow_architecture.png)

## Local Development

### Variant 1: Cargo

1. Install Rust and Cargo:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. ~/.bashrc
```

2. Clone the repository:

```bash
git clone https://github.com/HeliosShieldProject/master-backend-rust.git
cd master-backend-rust
```

3. Set the environment variables:

```bash
cp .env.example .env
# then edit the .env file with actual values
```

4. Run database:

```bash
docker-compose -f docker-compose-test up -d
```

5. Setup the database:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/download/v2.2.1/diesel_cli-installer.sh | sh && . ~/.bashrc
diesel setup
diesel migration run
```

> Also seed it with some data or use [psql with this sql](src/tests/e2e/sql/seed.sql)

6. Run the backend:

```bash
cargo run
```

### Variant 2: Docker

1. Clone the repository:

```bash
git clone
cd master-backend-rust
```

2. Set the environment variables:

```bash
cp .env.example .env
# then edit the .env file with actual values
```

3. Run database:

```bash
docker-compose up -d database
```

4. Setup the database:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/download/v2.2.1/diesel_cli-installer.sh | sh && . ~/.bashrc
diesel setup
diesel migration run
```

> Also seed it with some data or use [psql with this sql](src/tests/e2e/sql/seed.sql)

5. Run the backend:

```bash
docker-compose up -d master-backend
```

## Environment variables

```bash
mv .env.example .env
```

```bash
# backend
MASTER_BACKEND_PORT=1010
MASTER_BACKEND_HOST=localhost
MASTER_BACKEND_URL=${MASTER_BACKEND_HOST}:${MASTER_BACKEND_PORT}

# metrics
MASTER_METRICS_PORT=1111
MASTER_METRICS_HOST=localhost
MASTER_METRICS_URL=${MASTER_METRICS_HOST}:${MASTER_METRICS_PORT}

# database
DATABASE_PORT=5432
DATABASE_NAME=heliosdb
DATABASE_USER=helios
DATABASE_PASSWORD=very_strong_password
DATABASE_HOST=localhost
DATABASE_URL=postgresql://${DATABASE_USER}:${DATABASE_PASSWORD}@${DATABASE_HOST}:${DATABASE_PORT}/${DATABASE_NAME}

# jwt
JWT_ACCESS_SECRET=very_secret_access_token
JWT_REFRESH_SECRET=very_secret_refresh_token

# backend settings
SERVER_MODE=development
RUST_LOG=debug

# oauth
OAUTH_DISCORD_CLIENT_SECRET=secret_info
OAUTH_DISCORD_CLIENT_ID=secret_info
OAUTH_GITHUB_CLIENT_SECRET=secret_info
OAUTH_GITHUB_CLIENT_ID=secret_info
OAUTH_GOOGLE_CLIENT_SECRET=secret_info
OAUTH_GOOGLE_CLIENT_ID=secret_info

# resend.com - email service
RESEND_API_KEY=secret_info
```