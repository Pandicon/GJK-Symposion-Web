const ENDING_CHARACTERS = [
	'?',
	'(',
	'-',
	')',
	'_',
	'"',
	'/',
	'\\',
	'!',
	'.',
	'<',
	'>',
	':',
	';',
	'[',
	']',
	'{',
	'}',
	'%',
	'@',
	'£',
	'#',
	'$',
	'^',
	'&',
	'*'
];

/**
 * Parses the names in the table
 *
 * @param {Array<Array<string>>} table The table to parse
 * @return The parsed table
 * @customfunction
 */
function parse_table_data(table) {
	let end_index = table[1].findIndex((element) => element.trim() == '');
	if (end_index < 0) end_index = table[1].length;
	for (let row_i = 2; row_i < table.length; row_i += 1) {
		for (let col_i = 1; col_i < end_index; col_i += 1) {
			table[row_i][col_i] = parse_cell(table[row_i][col_i]).trim();
		}
	}
	return table;
}

/**
 * Parses the name in the cell
 *
 * @param {string} cell The cell to parse
 * @return The parsed cell
 * @customfunction
 */
function parse_cell(cell) {
	let s = cell.split('');
	let i = s.findIndex((character) => ENDING_CHARACTERS.includes(character));
	if (i < 0) return cell;
	s.length = i;
	return s.join('');
}
