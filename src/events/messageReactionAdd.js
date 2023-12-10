const { getPool } = require('../database');

module.exports = async (bot, reaction, user) => {
    console.log(user.username + "  reaction add:\n" + reaction);

    if (user.bot) return;

    const emoji = reaction.emoji;
    const message = reaction.message;

    let result;
    let emojisText = `<:${emoji.name}:${emoji.id}>`
    const pool = await getPool();
    if (emoji.id) {
        console.log("ID is cur");
        if (emoji.animated)
            emojisText.replace('<', '<a');

        console.log(message.guildId);
        console.log(message.channelId);
        console.log(message.id);
        console.log(emojisText);
        result = await pool.query(`select * from selfroles where guild_id = ${message.guildId} and channel_id = ${message.channelId} and message_id = ${message.id} and reaction = '${emojisText}' and active = true;`);
    } else {
        console.log("ID DOES NOT FOund");
        console.log(message.guildId);
        console.log(message.channelId);
        console.log(message.id);
        console.log(emoji.name);
        result = await pool.query(`select * from selfroles where guild_id = ${message.guildId} and channel_id = ${message.channelId} and message_id = ${message.id} and reaction = '${emoji.name}' and active = true;`);
    }
    const selfroles = result.rows;
    const member = message.guild.members.cache.get(user.id);

    // console.log(member);
    console.log(selfroles);

    roleIds = [];
    for (let i = 0; i < selfroles.length; i++) {
        const el = selfroles[i];
        console.log("IN FOR");

        roleIds.push(el.role_id);
    }
    console.log(roleIds);

    await member.roles.add(roleIds, `Роли-за-реакцию: Пользователь выбрал эти роли в ${message.channel.name}`)

};