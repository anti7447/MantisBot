module.exports = {
	data: {
		name: "selfrole",
        name_localizations: {ru: "роль-за-реакцию"},
        description: "Manage roles for reaction",
        description_localizations: {ru: "Управляйте ролями за реакцию!"},
        options: [
            {
                type: 1,
                name: "create",
                name_localizations: {ru: "создать"},
                description: "Creates a reaction under the message",
                description_localizations: {ru: "Creates a reaction under the message"},
                options: [
                    {
                        type: 7,
                        name: "channel",
                        name_localizations: {ru: "канал"},
                        description: "In which channel is the message to attach the reaction to?",
                        description_localizations: {ru: "В каком канале сообщение, к которому надо прикрепить реакцию?"},
                        required: true,
                        channel_types: [0, 5, 10, 11, 12, 15, 16]
                    },
                    {
                        type: 3,
                        name: "message_id",
                        name_localizations: {ru: "id_сообщения"},
                        description: "Specify the ID of the message to attach the reaction to",
                        description_localizations: {ru: "Укажите ID сообщения, к которому надо прикрепить реакцию"},
                        required: true
                    },
                    {
                        type: 3,
                        name: "reaction",
                        name_localizations: {ru: "реакция"},
                        description: "Specify the ID of the message to attach the reaction to",
                        description_localizations: {ru: "Укажите реакцию, которую надо прикрепить к сообщению"},
                        required: true
                    },
                    {
                        type: 8,
                        name: "role",
                        name_localizations: {ru: "роль"},
                        description: "Specify the ID of the message to attach the reaction to",
                        description_localizations: {ru: "Укажите роль, которая будет даваться"},
                        required: true
                    },
                ]
            },
            {
                type: 1,
                name: "update",
                name_localizations: {ru: "обновить"},
                description: "Creates a reaction under the message",
                description_localizations: {ru: "Creates a reaction under the message"},
                options: [
                    {
                        type: 7,
                        name: "channel",
                        name_localizations: {ru: "канал"},
                        description: "In which channel is the message to attach the reaction to?",
                        description_localizations: {ru: "В каком канале сообщение, к которому надо прикрепить реакцию?"},
                        required: true,
                        channel_types: [0, 5, 10, 11, 12, 15, 16]
                    },
                    {
                        type: 3,
                        name: "message_id",
                        name_localizations: {ru: "id_сообщения"},
                        description: "Specify the ID of the message to attach the reaction to",
                        description_localizations: {ru: "Укажите ID сообщения, к которому надо прикрепить реакцию"},
                        required: true
                    },
                    {
                        type: 3,
                        name: "reaction",
                        name_localizations: {ru: "реакция"},
                        description: "Specify the ID of the message to attach the reaction to",
                        description_localizations: {ru: "Укажите реакцию, которую надо прикрепить к сообщению"},
                        required: true
                    },
                    {
                        type: 8,
                        name: "role",
                        name_localizations: {ru: "роль"},
                        description: "Specify the ID of the message to attach the reaction to",
                        description_localizations: {ru: "Укажите роль, которая будет даваться"},
                        required: true
                    },
                ]
            },
            {
                type: 1,
                name: "browse",
                name_localizations: {ru: "просмотреть"},
                description: "Creates a reaction under the message",
                description_localizations: {ru: "Creates a reaction under the message"},
            },
            {
                type: 1,
                name: "delete",
                name_localizations: {ru: "удалить"},
                description: "Creates a reaction under the message",
                description_localizations: {ru: "Creates a reaction under the message"},
                options: [
                    {
                        type: 3,
                        name: "selfrole_id",
                        name_localizations: {ru: "id_роли-за-реакцию"},
                        description: "Specify the ID of the message to attach the reaction to",
                        description_localizations: {ru: "Укажите ID сообщения, к которому надо прикрепить реакцию"},
                        required: true
                    },
                ]
            },
        ],
        dm_permission: false
	},
	async execute(interaction) {

        const subCommandName = interaction.options.data[0].name;

        console.log(subCommandName);

        await require(`./selfrole/${subCommandName}Selfrole.js`)(interaction);
	},
};