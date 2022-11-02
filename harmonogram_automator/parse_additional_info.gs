/**
 * Parses the additional info from the table
 *
 * @param {Array<Array<string>>} table The table
 * @return The parsed info
 * @customfunction
 */
function parse_additional_info(table) {
	let first_row = table.shift();
	let data = {};
	let [
		name_index,
		title_index,
		annotation_index,
		lecturer_info_index,
		suitable_for_younger_index,
		korektura_index
	] = [-1, -1, -1, -1, -1, -1];
	for (const [cell_index, cell_raw] of first_row.entries()) {
		let value = cell_raw.toLowerCase().trim();
		switch (value) {
			case 'jméno hosta':
				name_index = cell_index;
				break;
			case 'název přednášky':
				title_index = cell_index;
				break;
			case 'anotace':
				annotation_index = cell_index;
				break;
			case 'medailonek':
				lecturer_info_index = cell_index;
				break;
			case 'vhodné pro mladší diváky?':
				suitable_for_younger_index = cell_index;
				break;
			case 'korektura':
				korektura_index = cell_index;
				break;
		}
	}
	for (const row of table) {
		let [
			lecturer,
			title,
			annotation,
			lecturer_info,
			suitable_for_younger,
			korektura
		] = [
			(row[name_index] ?? '').trim(),
			(row[title_index] ?? '').trim(),
			(row[annotation_index] ?? '').trim(),
			(row[lecturer_info_index] ?? '').trim(),
			(row[suitable_for_younger_index] ?? '').trim(),
			(row[korektura_index] ?? 'Provedena').trim()
		];
		data[lecturer] = {
			lecturer,
			title,
			annotation,
			lecturer_info,
			suitable_for_younger,
			korektura
		};
	}
	return data;
}
