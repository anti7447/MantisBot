const { getPool } = require("../database");

module.exports = async (bot, message) => {
    if(message.author.bot) return;

    const userId = message.author.id,
        username = message.author.username,
        guildId = message.guildId,
        guildname = message.guild.name;

    await checkUserTables(userId, username);
    if (message.inGuild) {
        await checkGuildTables(guildId, guildname);
        await checkMemberTables(userId, guildId);
    }
};

async function checkUserTables(userId, username) {
    const pool = await getPool();
    try {
        await pool.query('INSERT INTO users (id, username) VALUES ($1, \'$2\');'
            .replace('$1', userId)
            .replace('$2', username));
    } catch {}
}

async function checkGuildTables(guildId, guildname) {
    const pool = await getPool();
    try {
        await pool.query('INSERT INTO guilds (id, guildname, language) VALUES ($1, \'$2\', \'en\');'
            .replace('$1', guildId)
            .replace('$2', guildname));
    } catch {}
}

async function checkMemberTables(userId, guildId) {
    const pool = await getPool();
    try {
        await pool.query('INSERT INTO members (user_id, guild_id) VALUES ($1, $2);'
            .replace('$1', userId)
            .replace('$2', guildId));
    } catch {}
}