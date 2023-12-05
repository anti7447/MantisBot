const { getPool } = require("../database");
const fs = require('node:fs');

module.exports = async (bot) => {
    console.log(bot.user.username + " ready");
    doStartSQL();
};

async function doStartSQL() {
    fs.readFile('./sql/start.sql', 'utf8', async (err, data) => {
        if (err) throw err;
        const pool = await getPool();
        pool.query(data);
        console.log('Database is ready');
    });
}