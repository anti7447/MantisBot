const { tr } = require('../../../l10n');
const comPar = require('../../../command-params');
const { getPool } = require('../../../database');

module.exports = async (interaction) => {
		const guildId = interaction.guildId;
		const lang = (await comPar.language(guildId)).lang;

        const pool = await getPool();
        const result = await pool.query(`select * from selfroles where guild_id = ${guildId}`);
        const selfroles = result.rows;

        console.log(selfroles);

        let content = '';

        for (let i = 0; i < selfroles.length; i++) {
            const el = selfroles[i];
            content += `\`${el.id}\`\n`;
        }

		await interaction.reply(content);
};