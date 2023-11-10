import { Client } from "discord.js";

module.exports = async (bot: Client) => {
    console.log(bot.user?.username + " ready!");
}