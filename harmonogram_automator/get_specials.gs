const SPECIALS_TITLES = ['oběd', 'večeře'];

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
			data.push([[table[row_id][0]], curr]);
		}
	}
	return connect_times(data).map((element) => [
		element[0],
		element[1].charAt(0).toUpperCase() + element[1].slice(1)
	]);
}
