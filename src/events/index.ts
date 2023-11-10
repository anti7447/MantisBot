import { Client } from "discord.js"

module.exports = (bot: Client ) => {
    bot
        .on('ready', () => require('./ready')(bot));
}