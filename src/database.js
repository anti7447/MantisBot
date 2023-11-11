const { Pool } = require('pg')
const config = require('../config.json')

class Database {
	constructor(name) {
		this.name = name;
	  }
	async getPool() {
		const pool = new Pool(config.pool);
		return pool
	}
}

module.export = {Database};