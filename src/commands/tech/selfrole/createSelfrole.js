const { tr } = require('../../../l10n');
const { getPool } = require('../../../database');

const comPar = require('../../../command-params');
const errors = require('../../../error-embeds')

module.exports = async (interaction) => {
		const guildId = interaction.guildId;
		const lang = (await comPar.language(guildId)).lang;

        const options = interaction.options.data[0].options;
		const
			channelId = options[0].value,
			messageId = options[1].value,
			reaction  = options[2].value,
			roleId    = options[3].value;

        // console.log(options);

		const messageForSelfroles = await options[0].channel.messages.fetch(messageId)
			.then(msg => msg)
			.catch(err => null);

		if (!messageForSelfroles) {
			const emb = errors.customError(lang, 'Сообщение не найдено!', '`crSlr:FtchMsg` Если такое сообщение есть, обратитесь на [сервер бота](https://discord.gg/4SNv4E3eM3)');

    		return await interaction.reply({embeds: [emb], ephemeral: true});
		}

		const pool =  await getPool();

		const result = await pool.query(`select id from selfroles where guild_id = ${guildId}`)
		const selfIds = result.rows;

		let selfId = 1;

		console.log(selfIds);
		if (selfIds[0]) {
			console.log("SEFKLJFKESJFKN");
			for (let i = 0; i < selfIds.length; i++) {
				const el = selfIds[i].id;
				if (el !== (i+1)) {
					console.log("NOT ===");
					selfId = i+1;
					break;
				}
				selfId++;
			}
		}

		console.log(selfId);

		try {
			await pool.query(`INSERT INTO selfroles (id, guild_id, channel_id, message_id, reaction, role_id, active) VALUES (${selfId}, ${guildId}, ${channelId}, ${messageId}, '${reaction}', ${roleId}, True)`)
		} catch (err) {
			console.error(err);
			const emb = errors.customError(lang, 'Эта `роль-за-реакцию` уже создана!', '`crSlr:Pool` Если нет такой роли-за-реакцию, обратитесь на [сервер бота](https://discord.gg/4SNv4E3eM3)');

			return await interaction.reply({embeds: [emb], ephemeral: true}); 
		}

		try {
			await messageForSelfroles.react(reaction);
		} catch (error) {
			const emb = errors.customError(lang, 'Такой реакции не существует!', '`crSlr:React` Если такая реакцию существует, обратитесь на [сервер бота](https://discord.gg/4SNv4E3eM3)');

			return await interaction.reply({embeds: [emb], ephemeral: true});
		}



		await interaction.reply(tr(lang, 'ping-command'));
};