const SPECIALS_TITLES = ['oběd', 'večeře'];
const ANNOTATIONS = {
	oběd: 'Přijďte se posilnit do studentské kavárny nebo školní restaurace Scolarest! Jejich rozmanitá a pestrá kuchyně umožní každému zažehnat žízeň, utišit hlad a na pár okamžiků uniknout z víru dění.',
	večeře: 'Přijďte se posilnit do studentské kavárny nebo školní restaurace Scolarest! Jejich rozmanitá a pestrá kuchyně umožní každému zažehnat žízeň, utišit hlad a na pár okamžiků uniknout z víru dění.'
};
const ADDITIONAL_INFO = {};

/**
 * Gets the times of special stuff like lunch, dinner...
 *
 * @param {Array<Array<string>>} table The table
 * @return The specials times
 * @customfunction
 */
function get_specials(table) {
	let end_index = table[1].findIndex((element) => element.trim() == '');
	if (end_index < 0) return [];
	let prev = null;
	let data = [];
	for (let row_id = 0; row_id < table.length; row_id += 1) {
		let curr = table[row_id][end_index].trim().toLowerCase();
		if (curr == '' || !SPECIALS_TITLES.includes(curr)) {
			prev = null;
			continue;
		}
		if (curr == prev) {
			data[data.length - 1][0].push(table[row_id][0]);
		} else {
			data.push([
				[table[row_id][0]],
				curr,
				'Jídelna',
				ANNOTATIONS[curr] ?? '',
				ADDITIONAL_INFO[curr] ?? '',
				'ano'
			]);
		}
	}
	return connect_times(data).map((element) => {
		let time = element.shift();
		let event = element.shift();
		let room = element.shift();
		return [
			'!',
			time,
			room,
			event.charAt(0).toUpperCase() + event.slice(1),
			...element
		];
	});
}
