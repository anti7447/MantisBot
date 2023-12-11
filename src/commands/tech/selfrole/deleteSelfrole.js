const { tr } = require('../../../l10n');
const { getPool } = require('../../../database');

const comPar = require('../../../command-params');
const errors = require('../../../error-embeds')

module.exports = async (interaction) => {
		const guildId = interaction.guildId;
		const lang = (await comPar.language(guildId)).lang;

        const options = interaction.options.data[0].options;
		console.log(options);
		const selfroleId = options[0].value

		const pool = await getPool();
		const result = await pool.query(`select * from selfroles where id = ${Number(selfroleId)} and guild_id = ${guildId}`);
		const selfrole = result.rows[0];

		console.log(selfrole);

		try {
			if (selfrole.reaction.startsWith('<'))
				selfrole.reaction = selfrole.reaction.split(':')[2].replace('>', '');

			console.log(selfrole);
		} catch (error) {
			return await interaction.reply({
				ephemeral: true,
				embeds: [errors.customError(lang, 'Уже удалено!', `\`dlSlr:if\` если </selfrole browse:${interaction.id}> показывает, что этот ID существует, обратитесь на [сервер бота](https://discord.gg/4SNv4E3eM3)`)]
			});
		}

		try {
			
			
			const channel = await interaction.guild.channels.cache.get(selfrole.channel_id);
			const message = await channel.messages.cache.get(selfrole.message_id);
			const reaction = await message.reactions.resolve(selfrole.reaction)

			await reaction.users.remove(interaction.client.id)

			// console.log(channel);
			// console.log(message);

			// console.log(reaction);

		} catch (error) {
			console.error(error);
		}

		
		try {
			await pool.query(`delete from selfroles where id = ${Number(selfroleId)} and guild_id = ${guildId}`);
		} catch (error) {
			console.error(error);
			return await interaction.reply({
				ephemeral: true,
				embeds: [errors.customError(lang, 'Не существует!', `\`dlSlr:Pool\` если </selfrole browse:${interaction.id}> показывает, что этот ID существует, обратитесь на [сервер бота](https://discord.gg/4SNv4E3eM3)`)]
			});
		}

		await interaction.reply(tr(lang, 'ping-command'));
};