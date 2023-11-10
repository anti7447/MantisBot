const Discord = require('discord.js');
const { DISCORD_TOKEN } = require('../config.json');


const bot = new Discord.Client({
	intents: [
		Discord.GatewayIntentBits.Guilds,
		Discord.GatewayIntentBits.MessageContent,
		Discord.GatewayIntentBits.GuildMessages]
});

require('./events')(bot);

bot.login(DISCORD_TOKEN);