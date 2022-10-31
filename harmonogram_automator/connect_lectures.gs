/**
 * Connects the multicell lectures
 *
 * @param {Array<Array<string>>} table The table to connect
 * @return The connected table
 * @customfunction
 */
function connect_lectures(table) {
	let end_index = table[0].length;
	for (let col_id = 0; col_id < end_index; col_id += 1) {
		let prev = null;
		for (let row_id = 0; row_id < table.length; row_id += 1) {
			if (
				table[row_id][col_id].trim() != '-' &&
				table[row_id][col_id].trim() != ''
			)
				prev = table[row_id][col_id];
			else if (table[row_id][col_id].trim() == '' && prev != null)
				table[row_id][col_id] = prev;
		}
	}
	return table;
}
