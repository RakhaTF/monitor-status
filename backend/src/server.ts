import express, { Request, Response } from 'express';
import mysql from 'mysql2/promise';
import cors from 'cors';
import dotenv from 'dotenv';

dotenv.config();

const app = express();
app.use(cors());

// Connection to your EXISTING database
const pool = mysql.createPool({
    host: process.env.DB_HOST || 'host.docker.internal',
    port: Number(process.env.DB_PORT) || 3306,
    user: process.env.DB_USER || 'root',
    password: process.env.DB_PASSWORD || '',
    database: process.env.DB_NAME || 'monitoring_db',
    waitForConnections: true,
    connectionLimit: 10,
    queueLimit: 0
});

interface ServiceRow extends mysql.RowDataPacket {
    id: number;
    project_name: string;
    last_update: Date;
    status: 'ok' | 'down';
}

app.get('/api/status', async (req: Request, res: Response) => {
    try {
        // Queries the table created by your other services
        const [rows] = await pool.query<ServiceRow[]>('SELECT * FROM service_status');

        const processed = rows.map((service) => {
            const lastUpdate = new Date(service.last_update);
            const now = new Date();
            const diffSeconds = (now.getTime() - lastUpdate.getTime()) / 1000;

            let displayStatus = 'operational';

            if (service.status === 'down') {
                displayStatus = 'outage';
            } else if (diffSeconds > 120) {
                // Logic: Stale if older than 2 mins
                displayStatus = 'stale';
            }

            // Cosmetic ID generation
            const fakeHexId = Buffer.from(`svc-${service.id}`).toString('hex').substring(0, 8);

            return {
                id: service.id,
                displayId: fakeHexId,
                name: service.project_name,
                last_update: service.last_update,
                status: displayStatus
            };
        });

        res.json(processed);
    } catch (err) {
        console.error("Database Error:", err);
        res.status(500).json({ error: 'Could not fetch status data' });
    }
});

const PORT = 3000;
app.listen(PORT, () => console.log(`Monitor API running on port ${PORT}`));