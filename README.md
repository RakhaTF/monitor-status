# Microservice Status Monitor

A high-fidelity, read-only status dashboard for microservice infrastructure. It visualizes service health (Operational, Outage, Stale) based on heartbeat timestamps stored in your existing external MySQL database.

## Architecture

* **Frontend**: Pure HTML/CSS/JS (served via Nginx).
    * **Style**: Cyberpunk/Dark UI.
    * **Timezone**: Automatically converts UTC timestamps to **UTC+7** (via Moment.js).
* **Backend**: Node.js (TypeScript) API.
    * **Role**: Read-only fetcher. Does not modify data.
    * **Connection**: configured to connect to the **host machine's** MySQL instance (`host.docker.internal`).
* **Infrastructure**: Docker Compose (Nginx + Node API).

## Project Structure

Ensure your directory matches this tree:

```text
/monitor-dashboard
├── docker-compose.yml
├── /frontend
│   └── index.html             # Dashboard UI (includes Moment.js CDN)
└── /backend
    ├── Dockerfile             # Multi-stage Docker build
    ├── package.json           # Node dependencies (mysql2, express)
    ├── tsconfig.json          # TypeScript config (CommonJS/Node)
    └── /src
        └── server.ts          # API Logic