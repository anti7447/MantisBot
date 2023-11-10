const { Pool } = require('pg')


export async function getPool() {
    const pool = new Pool({
		host: 'localhost',
		user: 'mantis',
		password: '132435',
		database: 'mantisdb',
		connectionString: 'postgres://mantis:132435@localhost/mantisdb',
		max: 30,
		idleTimeoutMillis: 30000,
		connectionTimeoutMillis: 2000
	});
	return pool
}