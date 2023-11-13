const { SlashCommandBuilder } = require('discord.js');

module.exports = {
	data: {
        name: "language",
        name_localizations: {ru: "язык"},
        description: "changes the language",
        description_localizations: {ru: "Сменяет язык бота"},
        options: [
            {
                type: 3,
                name: "language",
                name_localizations: {ru: "язык"},
                description: "Select the language the bot into (this does not affect the names and descriptions of slash commands)",
                description_localizations: {ru: "Выберите язык бота (это не влияет на имена и описания слэш-команд)"},
                required: true,
                choices: [
                    {
                        name: "English",
                        name_localizations: {ru: "Английский"},
                        value: "en"
                    },
                    {
                        name: "Russian",
                        name_localizations: {ru: "Русский"},
                        value: "ru"
                    },
                ]
            }
        ],
        dm_permission: false
    },
	async execute(interaction) {
		await interaction.reply('Pong!');
	},
};