const l10n = require('../locales.json');

const tr = (lang, param, args) => {
	let str = l10n[lang][param];
	
	return replace_arguments(str, args);
}

function replace_arguments(str, args) {
	let from = str.indexOf('{');
	if (from == -1)
		return str;
	let to = str.indexOf('}');
	let basis_arg = str.slice(from, to);
	let name_arg = basis_arg.slice(2, basis_arg.indexOf(' '));
	let text_basis_arg = basis_arg.slice(2+name_arg.length+4);

	let replaced_arg = '';
	if (text_basis_arg === '') {
		replaced_arg = args[name_arg];
	} else {
		let arr_basis_arg = text_basis_arg.split(' ');
		let arg = {};
		for (let i = 0; i < arr_basis_arg.length; i++) {
			let el = arr_basis_arg[i];
			if ((i % 2) == 0) {
				if (el.startsWith('*')) {
					arg['default'] = arr_basis_arg[i+1];
					el = el.slice(1);
				}
				arg[el.slice(1,-1)] = arr_basis_arg[i+1];
			}
		};
	

		replaced_arg = arg[args[name_arg]];
		if (replaced_arg === undefined)
			replaced_arg = arg['default'];
	}
	let final_str = str.replace(basis_arg+'}', replaced_arg);

	return replace_arguments(final_str, args);
}

module.exports.tr = tr;