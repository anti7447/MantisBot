module.exports = (bot) => {
    bot
        .on('ready', () => require('./ready')(bot))  
        .on('interactionCreate', (interaction) => require('./interactionCreate')(bot, interaction));
};
