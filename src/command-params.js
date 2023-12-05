const { getPool } = require('./database')
const config = require('../config.json')

async function language(guildId) {
	const pool = await getPool();

    const result = await pool.query('select language from guilds where id = $id;'.replace('$id', guildId));
	let lang = result.rows[0].language;
	return {lang: lang}
}

module.exports.language = language;