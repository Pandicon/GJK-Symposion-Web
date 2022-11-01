const DAYS_CONVERT = {
	ST: 'streda',
	ČT: 'ctvrtek',
	PÁ: 'patek'
};

/**
 * Connects the time ranges, taking the start time of the first time and the end time of the last time
 *
 * @param {Array<Array<string>, string>} times The times
 * @return The connected times
 * @customfunction
 */
function connect_times(times) {
	return times.map((element) => {
		let times = element.shift();
		if (times.length == 0) return '';
		if (times.length == 1) {
			let time = times[0];
			for (const day in DAYS_CONVERT) {
				time = time.replace(day, DAYS_CONVERT[day]);
			}
			return [time, ...element];
		}
		let split_time = times[0].split(' ');
		let day = split_time.shift().trim().toUpperCase();
		let [start_time, _first_end_time] = split_time
			.join(' ')
			.trim()
			.split('-')
			.map((e) => e.trim());
		let split_time_last = times[times.length - 1].split(' ');
		split_time_last.shift();
		let [_last_start_time, end_time] = split_time_last
			.join(' ')
			.trim()
			.split('-')
			.map((e) => e.trim());
		return [
			`${DAYS_CONVERT[day]} ${[start_time, end_time].join(' - ')}`,
			...element
		];
	});
}
