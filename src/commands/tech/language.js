const { getPool } = require('../../database');
const { tr } = require('../../l10n');

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
        const guildId = interaction.guildId;
        const option = interaction.options.get('language', true);
        const pool = await getPool();

        const result = await pool.query('UPDATE guilds SET language=\'$1\' WHERE id = \'$2\' RETURNING language'
            .replace('$1', option.value)
            .replace('$2', guildId));

        const lang = result.rows[0].language;

        const embed = {
            title: tr(lang, 'lang-command-ttl', {lang: lang}),
            description: tr(lang, 'lang-command-dst', {lang: lang}),
            color: 0x44ea62
        }

		await interaction.reply({embeds: [embed]});
	},
};