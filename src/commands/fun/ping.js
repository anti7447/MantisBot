const { SlashCommandBuilder } = require('discord.js');
const { tr } = require('../../l10n');
const comPar = require('../../command-params')

module.exports = {
	data: {
		name: "ping",
        name_localizations: {ru: "пинг"},
        description: "ping's command",
        description_localizations: {ru: "Команда пинга"}
	},
	async execute(interaction) {
		const guildId = interaction.guildId;
		const lang = (await comPar.language(guildId)).lang;

		await interaction.reply(tr(lang, 'ping-command'));
	},
};