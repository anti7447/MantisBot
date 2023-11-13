const { SlashCommandBuilder } = require('discord.js');

module.exports = {
	data: {
		name: "ping",
        name_localizations: {ru: "пинг"},
        description: "ping's command",
        description_localizations: {ru: "Команда пинга"}
	},
	async execute(interaction) {
		await interaction.reply('Pong!');
	},
};