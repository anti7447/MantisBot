module.exports = (bot) => {
    bot
        .on('ready', () => require('./ready')(bot))
        .on('interactionCreate', (interaction) => require('./interactionCreate')(bot, interaction))
        .on('messageCreate', (message) => require('./messageCreate')(bot, message))
        .on('messageReactionAdd', (reaction, user) => require('./messageReactionAdd')(bot, reaction, user))
        .on('messageReactionRemove', (reaction, user) => require('./messageReactionRemove')(bot, reaction, user));
};
