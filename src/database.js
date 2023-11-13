const { Pool } = require('pg')
const config = require('../config.json')

// var database = class Database {
// 	constructor(name) {
// 		this.name = name;
// 	  }
// 	async getPool() {
// 		const pool = new Pool(config.pool);
// 		return pool
// 	}
// }

// var getPool = async function() {
// 	const pool = new Pool(config.pool);
// 	return pool
// }

// let db = new Database("asd");

// (async () => {
// 	console.log("DB");
// 	const pool = await getPool();

// 	console.log(poo;);

// 	const result = await pool.query('select id from guilds;')
// 	console.log(result.rows)
// })()

module.exports.getPool = async () => {
	const pool = new Pool(config.pool);
	return pool
};