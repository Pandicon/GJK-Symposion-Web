/**
 * Pairs up the time and the lecture
 *
 * @param {Array<Array<string>>} table The table
 * @return The paired up data
 * @customfunction
 */
function pair_time_lecture(table) {
	let data = [];
	let end_index = table[1].findIndex((element) => element.trim() == '');
	if (end_index < 0) end_index = table[1].length;
	for (let col_id = 1; col_id < end_index; col_id += 1) {
		let prev = null;
		for (let row_id = 2; row_id < table.length; row_id += 1) {
			let curr = table[row_id][col_id].trim();
			if (curr == '') {
				prev = null;
				continue;
			}
			if (curr == prev) {
				data[data.length - 1][0].push(table[row_id][0]);
			} else {
				prev = curr;
				data.push([[table[row_id][0]], curr]);
			}
		}
	}
	return connect_times(data);
}
