/**
 * Parses the harmonogram data
 *
 * @param {Array<Array<string>>} harmonogram The harmonogram we get from the team
 * @return The harmonogram for the web
 * @customfunction
 */
function harmonogram(harmonogram) {
	let final_data = [];
	let table = cut_table(harmonogram);
	if (table.length < 2) return table;
	let specials_times = get_specials(table);
	let connected_table = connect_lectures(table);
	let parsed_table = parse_table_data(connected_table);
	let lecture_times = pair_time_lecture(parsed_table);
	final_data = [...specials_times, ...lecture_times];
	return final_data;
}
