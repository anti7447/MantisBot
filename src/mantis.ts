import { Message } from "discord.js";

const Discord = require('discord.js');
const { DISCORD_TOKEN } = require('../config.json');

const bot = new Discord.Client({
	intents: [
		Discord.GatewayIntentBits.Guilds,
		Discord.GatewayIntentBits.MessageContent,
		Discord.GatewayIntentBits.GuildMessages]
});

// bot.on('ready', () => {
// 	console.log(`Logged in as ${bot.user?.tag}!`);
// });


require('./events')(bot);


bot.login(DISCORD_TOKEN);