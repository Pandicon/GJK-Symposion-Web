/**
 * Pairs up the time and the lecture
 *
 * @param {Array<Array<string>>} table The table
 * @return The paired up data
 * @customfunction
 */
function pair_time_lecture_room(table) {
	let data = {};
	let end_index = table[1].findIndex((element) => element.trim() == '');
	if (end_index < 0) end_index = table[1].length;
	let first_row = table[0];
	let prev_cell = null;
	for (let i = 0; i < first_row.length; i += 1) {
		let curr = first_row[i].trim();
		if (curr == '') {
			if (prev_cell == null) continue;
			first_row[i] = prev_cell;
			continue;
		}
		prev_cell = curr;
	}
	first_row.shift();
	for (let col_id = 1; col_id < end_index; col_id += 1) {
		let col_index = col_id - 1;
		let col_name = `${first_row[col_index]}`;
		if (!Object.keys(data).includes(col_name)) data[col_name] = [];
		let prev = null;
		for (let row_id = 2; row_id < table.length; row_id += 1) {
			let curr = table[row_id][col_id].trim();
			if (curr == '') {
				prev = null;
				continue;
			}
			if (curr == prev) {
				data[col_name][data[col_name].length - 1][0].push(
					table[row_id][0]
				);
			} else {
				prev = curr;
				data[col_name].push([
					[table[row_id][0]],
					curr,
					table[1][col_id]
				]);
			}
		}
	}
	for (const key in data) {
		data[key] = connect_times(data[key]).map((element) => {
			let time = element.shift();
			let lecturer = element.shift();
			let room = element.shift();
			return [lecturer, time, room];
		});
	}
	return data;
}
