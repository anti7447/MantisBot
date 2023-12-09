const { tr } = require("./l10n");

/** 
 * **`DEVELOP ONLY`**
 * @param lang guild's language
 * @param desc embed's discription:
 * what kind of mistake is this
 * @param tech embed's footer:
 * brief technical information for better error detection and correction
 * @returns embed's json
 */
function customError(lang, desc, tech) {
	const errorEmbed = {
        title: tr(lang, 'error-ttl'),
        description: `${desc}\n\n${tech}`,
        color: 0xff0000,
        // footer: {text: tech}
    };
	return errorEmbed
}

module.exports.customError = customError;