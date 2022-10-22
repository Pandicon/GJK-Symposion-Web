#include <chrono>
#include <iostream>
#include <queue>
#include <sstream>
#include <thread>
#include "./config.hpp"
#include "./fetch.hpp"
#include "./table.hpp"
#include "./wserver.hpp"

enum class fetch_task_t {
	NONE, SHEET_TEST
};
const char *fetch_task_strs[] {
	"none",
	"sheet_test",
};
fetch_task_t current_fetch_task;
std::queue<fetch_task_t> fetch_tasks;
api_server::cache<std::string> sheet_test_cache;

api_server::http_response reqest_callback(const api_server::http_request &req) {
	api_server::http_response res;
	res.close = true;
	res.code = 404;
	res.content_type = "text/json";
	res.content = "{\"data\":null,\"error\":\"Unknown url.\"}";
	if (req.method != api_server::http_request::GET) {
		res.code = 403;
		res.content = "{\"error\":\"Any request method other than GET isn't allowed.\",\"data\":null}";
	} else if (req.url == "/secret") {
		res.code = 200;
		res.content = "{\"data\":{\"secret_message\":\"Nazdárek!\"},\"error\":null}";
	} else if (req.url == "/sheet_test") {
		if (sheet_test_cache.get().empty()) {
			res.code = 500;
			res.content = "{\"data\":null,\"error\":\"Could not retrieve test sheet.\"}";
		} else {
			res.code = 200;
			res.content = "{\"data\":{\"sheet\":" + sheet_test_cache.get() + ","
				"\"last_updated\":\"" + std::to_string(sheet_test_cache.get_last_update_time_since_epoch()) + "\"},\"error\":null}";
		}
		if (sheet_test_cache.should_update(std::chrono::minutes(30))) {
			fetch_tasks.emplace(fetch_task_t::SHEET_TEST);
		}
	}
	return res;
}

int main(int, char**) {
    api_server::load_cfg("./api_server.cfg");
	asio::io_service io_service;
	std::istringstream portss(api_server::get_config_or("port", "443"));
	unsigned short port;
	portss >> port;
	if (portss.fail() || !portss.eof()) {
		std::cerr << "[error]: config: port has to be a number 0-65536" << std::endl;
		return -1;
	}
	api_server::wserver server(io_service, port, reqest_callback);
	api_server::sheet_fetcher fetcher(io_service);
	fetch_tasks.push(fetch_task_t::SHEET_TEST);
	while (true) {
		io_service.poll();
		if (fetcher.is_done()) {
			switch (current_fetch_task) {
			case fetch_task_t::NONE: break;
			case fetch_task_t::SHEET_TEST:{
				if (fetcher.is_success()) {
					std::ostringstream oss;
					oss << "[";
					bool first_row = true;
					for (const auto &row : fetcher.get_sheet()) {
						oss << (first_row ? "[" : ",[");
						first_row = false;
						bool first_cell = true;
						for (const auto &cell : row) {
							if (!first_cell) oss << ',';
							first_cell = false;
							oss << '"' << cell << '"';
						}
						oss << "]";
					}
					oss << "]";
					sheet_test_cache.update(oss.str());
					std::cout << "\x1b[1m\x1b[96m[tasks::fetcher]: fetched sheet test\x1b[0m" << std::endl;
				} else {
					std::cout << "\x1b[1m\x1b[91m[tasks::fetcher::error]: failed to fetch sheet test\x1b[0m" << std::endl;
				}
				break;
			}
			default: break;
			}
			current_fetch_task = fetch_task_t::NONE;
			if (!fetch_tasks.empty()) {
				std::string sheet = api_server::get_config_or("schedule_sheet", "");
				current_fetch_task = fetch_tasks.front();
				fetch_tasks.pop();
				switch (current_fetch_task) {
				case fetch_task_t::SHEET_TEST:
					if (!sheet_test_cache.should_update(std::chrono::seconds(10))) {
						current_fetch_task = fetch_task_t::NONE;
						goto skip_fetch_task_assign;
					}
					if (!sheet.empty())
						fetcher.start_fetch(sheet, "test");
					break;
				default: break;
				}
				std::cout << "[tasks::fetcher]: assigned fetcher task " << fetch_task_strs[static_cast<size_t>(fetch_tasks.front())] << std::endl;
			}
		skip_fetch_task_assign:;
		}
		std::this_thread::sleep_for(std::chrono::milliseconds(50));
	}
	return 0;
}
