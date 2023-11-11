const { SlashCommandBuilder } = require('discord.js');

module.exports = {
	data: new SlashCommandBuilder()
		.setName('ping')
		.setNameLocalization("ru", "пинг")
		.setDescription('Replies with Pong!')
		.setDescriptionLocalization("ru", "Отвечает Понг!"),
	async execute(interaction) {
		await interaction.reply('Pong!');
	},
};