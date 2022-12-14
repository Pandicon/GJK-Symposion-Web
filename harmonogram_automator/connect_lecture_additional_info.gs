/**
 * Connects additional info and base lecture info
 *
 * @param {Object<Array<Array<string>>>} lectures The lectures
 * @param {Object<Array<Object>>} additional_info The additional info
 * @return The connected info
 * @customfunction
 */
function connect_lecture_additional_info(lectures, additional_info) {
	let data = [];
	for (const lecture of lectures) {
		let lecturer = lecture[0].trim();
		let [lecturer_name_full, additional_lecturer_info] = additional_info[
			lecturer
		]
			? [lecturer, additional_info[lecturer]]
			: find_lecturer_inefficient(additional_info, lecturer);
		if (!additional_lecturer_info) continue;
		lecture[0] = lecturer_name_full;
		let {
			title,
			annotation,
			lecturer_info,
			suitable_for_younger,
			korektura,
			...other
		} = additional_lecturer_info;
		data.push([
			...lecture,
			title,
			annotation,
			lecturer_info,
			suitable_for_younger,
			korektura
		]);
	}
	return data;
}

/**
 * Connects additional info and base lecture info
 *
 * @param {Array<Array<string>>} additional_info_field The additional information about all people from a field
 * @param {Object<Array<Object>>} additional_info The additional info
 * @return The connected info
 * @customfunction
 */
function find_lecturer_inefficient(additional_info_field, lecturer) {
	for (const key in additional_info_field) {
		if (key.toLowerCase().includes(lecturer.trim().toLowerCase()))
			return [key, additional_info_field[key]];
	}
	console.log('Lecturer ', lecturer.trim().toLowerCase(), ' not found');
	return [null, null];
}
