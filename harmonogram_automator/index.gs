/**
 * Parses the harmonogram data
 *
 * @param {Array<Array<string>>} harmonogram The harmonogram we get from the team
 * @param {Array<Array<string>>} humanities The humanities people list
 * @param {Array<Array<string>>} prirodovedci The scientists list
 * @param {Array<Array<string>>} praktici The practical people list
 * @return The harmonogram for the web
 * @customfunction
 */
function harmonogram(harmonogram, prirodovedci, humanities, praktici) {
	let final_data = [];
	let table = cut_table(harmonogram);
	if (table.length < 2) return table;
	let cut_humanties = cut_table(humanities);
	let cut_prirodovedci = cut_table(prirodovedci);
	let cut_praktici = cut_table(praktici);
	let specials_times = get_specials(table);
	let connected_table = connect_lectures(table);
	let parsed_table = parse_table_data(connected_table);
	let lecture_times_rooms = pair_time_lecture_room(parsed_table);
	console.log(lecture_times_rooms);
	let additional_info = {
		...parse_additional_info(cut_prirodovedci),
		...parse_additional_info(cut_humanties),
		...parse_additional_info(cut_praktici)
	};
	let final_lectures_data = connect_lecture_additional_info(
		lecture_times_rooms,
		additional_info
	);
	final_data = [
		...specials_times,
		...final_lectures_data.filter(
			(lecture) => lecture[7].toLowerCase() == 'provedena'
		)
	];
	return final_data;
}
