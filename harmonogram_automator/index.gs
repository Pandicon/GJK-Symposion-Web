/**
 * Parses the harmonogram data
 *
 * @param {Array<Array<string>>} harmonogram The harmonogram we get from the team
 * @return The harmonogram for the web
 * @customfunction
 */
function harmonogram(harmonogram) {
	let table = cut_table(harmonogram);
	if (table.length < 2) return table;
	let parsed_table = parse_table_data(table);
	return parsed_table;
}
