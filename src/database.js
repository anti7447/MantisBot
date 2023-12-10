const { Pool } = require('pg')
const config = require('../config.json')

/**
 * Get a pool for sql-reqests
 * 
 * @returns pool
 */
async function getPool() {
	const pool = new Pool(config.pool);
	return pool
}

module.exports.getPool = getPool;