#include "table.hpp"

#include <cctype>
#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <unordered_map>
#include <unordered_set>

namespace api_server {
	void string_tolower_no_whitespace(std::string &s) {
		auto out_it = s.begin();
		for (auto c = s.begin(); c != s.end(); c++) { // <- c++ :)
			if (!std::isspace(*c)) {
				*(out_it++) = std::tolower(static_cast<unsigned char>(*c));
			}
		}
		s.erase(out_it, s.end());
	}
	void extract_times(const std::string &when, unsigned int &d, unsigned int &sh, unsigned int &sm, unsigned int &eh, unsigned int &em) {
		if (when.size() < 1) {
			std::cerr << "[schedule]: empty time!" << std::endl;
			d = 0;
			return;
		}
		switch (when[0])
		{
		case 's': d = 0; break;
		case 'c':
		case '\xc4': d = 1; break;
		case 'p': d = 2; break;
		default: std::cerr << "[schedule]: invalid day! (" << when << ")" << std::endl; d = 1; break; // probably č for "čtvrtek" with some weird encoding
		}
		sh = sm = eh = em = 0;
		unsigned int *times[] = { &sh, &sm, &eh, &em };
		unsigned int **i = times;
		bool last_num = false;
		for (char c : when) {
			if (c >= '0' && c <= '9') {
				**i = (**i) * 10 + static_cast<unsigned int>(c-'0');
				last_num = true;
			} else {
				if (last_num) {
					i++;
					if (i-times >= 4) {
						break;
					}
					last_num = false;
				}
			}
		}
	}
	std::string json_str(std::string s) {
		for (auto it = s.find_first_of("\\\""); it != std::string::npos; it = s.find_first_of("\\\"", it + 2)) {
			s.insert(it, 1, '\\');
		}
		return "\""+s+"\"";
	}

	using time_range_t = std::pair<unsigned int, unsigned int>;
	struct lecture_t {
		std::string lecturer;
		std::string title;
		time_range_t time;
		size_t annotation_id;
		bool for_younger;
	};

	schedule::schedule(const std::vector<std::vector<std::string>> &sheet) {
		std::set<std::string> places;
		using time_frame_t = std::unordered_map<std::string, lecture_t>;
		std::vector<std::map<unsigned int, time_frame_t>> lecture_tables{{},{},{}};
		annotations.push_back("null");
		auto i = sheet.begin();
		for (i++ /* skip table header */; i != sheet.end(); i++) {
			if (i->size() < 7) {
				std::cerr << "[schedule]: invalid row: " << (i-sheet.begin()) << std::endl;
				continue;
			}
			lecture_t lecture;
			lecture.lecturer = (*i)[0];
			std::string when = (*i)[1]; // I need to know format first
			const std::string &where = (*i)[2];
			lecture.title = (*i)[3];
			if (lecture.title.empty() || lecture.title.empty())
				continue; // does not count :)
			const std::string &annotation = (*i)[4];
			const std::string &about_lecturer = (*i)[5];
			std::string for_younger_str = (*i)[6];
			// handle multiple "yes" answers for "for younger" field
			string_tolower_no_whitespace(for_younger_str);
			lecture.for_younger = std::set<std::string>{"ano", "jo", "a"}.contains(for_younger_str);
			lecture.annotation_id = annotations.size();
			annotations.push_back(annotation.empty() && about_lecturer.empty() ? "null" : json_str(annotation + "\n\n" + about_lecturer));
			// parse "when"
			string_tolower_no_whitespace(when);
			unsigned int day, starth, startm, endh, endm;
			extract_times(when, day, starth, startm, endh, endm);
			lecture.time = time_range_t(starth*60+startm, endh*60+endm);
			// write to table
			lecture_tables[day][lecture.time.first].emplace(where, lecture);
		}
		// load place annotations
		std::unordered_map<std::string, size_t> place_annotations;
		for (i = sheet.begin(); i != sheet.end() && i->size() > 9; i++) {
			if (!(*i)[8].empty() && !(*i)[9].empty()) {
				place_annotations.emplace((*i)[8], annotations.size());
				annotations.push_back((*i)[9]);
			}
		}
		// generate
		day_jsons = std::vector<std::string>(3, std::string());
		for (unsigned int day = 0; day < 3; day++) {
			const auto &d = lecture_tables[day];
			std::vector<std::string> times;
			std::set<std::string> places;
			std::vector<std::vector<const lecture_t *>> table;
			{
				// get all time splits and places
				std::set<unsigned int> time_splits;
				for (const auto &tf : d) {
					for (const auto &lec : tf.second) {
						time_splits.insert({ lec.second.time.first, lec.second.time.second });
						if (lec.second.lecturer != "!")
							places.emplace(lec.first);
					}
				}
				// make left table header, prepare map to indices
				std::unordered_map<unsigned int, unsigned int> ts2i;
				unsigned int prev = 0;
				unsigned int j = 0;
				for (unsigned int ts : time_splits) {
					ts2i.emplace(ts, j++);
					if (prev) {
						times.push_back(std::to_string(prev / 60) + ":" + (prev % 60 < 10 ? "0" : "") + std::to_string(prev % 60) + " - " +
										std::to_string(ts / 60) + ":" +   (ts % 60 < 10 ? "0" : "") +   std::to_string(ts % 60));
					}
					prev = ts;
				}
				// prepare map to indices for top header
				std::unordered_map<std::string, unsigned int> pl2i;
				j = 0;
				for (const std::string &place : places) { pl2i.emplace(place, j++); }
				// make the table
				table = std::vector<std::vector<const lecture_t *>>(times.size(), std::vector<const lecture_t *>(places.size(), nullptr));
				for (const auto &tf : d) {
					for (const auto &lec : tf.second) {
						if (lec.second.lecturer == "!") { // special events like diner...
							for (j = ts2i[lec.second.time.first]; j < ts2i[lec.second.time.second]; j++) {
								for (auto &p_lec : table[j]) {
									p_lec = &(lec.second);
								}
							}
						} else {
							for (j = ts2i[lec.second.time.first]; j < ts2i[lec.second.time.second]; j++) {
								table[j][pl2i[lec.first]] = &(lec.second);
							}
						}
					}
				}
			}
			// filter empty rows
			// TODO: optimize in a way std::remove_if works
			for (unsigned int j = 0; j < table.size(); j++) {
				if (table[j].size() == places.size() && std::all_of(table[j].begin(), table[j].end(), [](const lecture_t *p_lec) { return p_lec == nullptr; })) {
					table.erase(table.begin() + j);
					times.erase(times.begin() + j);
				}
			}
			// prepare annotation_indices
			annotation_indices.emplace_back(table.size() + 1, std::vector<size_t>(places.size() + 1, 0));
			// generate the json and save annotation_indices
			std::ostringstream oss;
			oss << "[[null";
			annotation_indices[day][0][0] = 0;
			unsigned int k = 1;
			for (const auto &place : places) {
				oss << ",{\"lecturer\":\"\",\"title\":" << json_str(place) << ",\"for_younger\":false}";
				auto pai = place_annotations.find(place);
				annotation_indices[day][0][k++] = pai == place_annotations.end() ? 0 : pai->second;
			}
			oss << "]";
			std::unordered_set<const lecture_t *> written;
			for (unsigned int j = 0; j < table.size(); j++) {
				oss << ",[";
				const auto &time = times[j];
				oss << "{\"lecturer\":\"\",\"title\":" << json_str(time) << ",\"for_younger\":false}";
				annotation_indices[day][j][0] = 0;
				for (k = 0; k < table[j].size(); k++) {
					const lecture_t *l = table[j][k];
					if (l == nullptr) {
						oss << ",null";
						annotation_indices[day][j+1][k+1] = 0;
					} else if (!written.contains(l)) {
						oss << ",{\"lecturer\":" << (l->lecturer == "!" ? "\"\"" : json_str(l->lecturer)) << ",\"title\":" << json_str(l->title) <<
							",\"for_younger\":" << (l->for_younger ? "true" : "false");
						size_t span = 0;
						for (unsigned int m = j; m < table.size() && table[m][k] == l; m++) { span++; }
						if (span > 1) { oss << ",\"row_span\":" << span; }
						span = 0;
						for (unsigned int m = k; m < table[j].size() && table[j][m] == l; m++) { span++; }
						if (span > 1) { oss << ",\"col_span\":" << span; }
						oss << "}";
						written.insert(l);
					}
					if (l != nullptr)
						annotation_indices[day][j+1][k+1] = l->annotation_id;
				}
				oss << "]";
			}
			oss << "]";
			day_jsons[day] = oss.str();
		}
	}
}
