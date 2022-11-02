/**
 * Cuts out the relevant data from the table, leaving out anything after an empty column and an empty row
 *
 * @param {Array<Array<string>>} table The table to cut
 * @return The cut table
 * @customfunction
 */
function cut_table(table) {
	let cut_table = [];
	for (const row of table) {
		if (
			row.filter(
				(element) =>
					!(
						typeof element === 'string' || element instanceof String
					) || element.trim() != ''
			).length == 0 ||
			row.findIndex(
				(element) =>
					(typeof element === 'string' ||
						element instanceof String) &&
					element.toLowerCase().trim().startsWith('počítáme')
			) != -1
		)
			break;
		cut_table.push(row);
	}
	if (cut_table.length == 0) return cut_table;
	for (let i = 0; i < cut_table[0].length; i += 1) {
		let empty = true;
		for (const row of table) {
			if (row[i].trim() != '') {
				empty = false;
				break;
			}
		}
		if (empty) {
			for (const row of table) {
				row.length = i;
			}
			break;
		}
	}
	return cut_table;
}
