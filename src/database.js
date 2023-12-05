const { Pool } = require('pg')
const config = require('../config.json')

async function getPool() {
	const pool = new Pool(config.pool);
	return pool
}

module.exports.getPool = getPool;