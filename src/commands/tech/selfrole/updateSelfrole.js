const { tr } = require('../../../l10n');
const comPar = require('../../../command-params')

module.exports = async (interaction) => {
		const guildId = interaction.guildId;
		const lang = (await comPar.language(guildId)).lang;

        const options = interaction.options.data[0].options;

        console.log(options);

		await interaction.reply(tr(lang, 'ping-command'));
};