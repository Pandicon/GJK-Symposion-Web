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
	NONE, SCHEDULE
};
const char *fetch_task_strs[] {
	"none", "schedule"
};
fetch_task_t current_fetch_task;
std::queue<fetch_task_t> fetch_tasks;
api_server::cache<api_server::schedule> schedule_cache;

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
	} else if (req.url == "/streda") {
		if (schedule_cache.get().day_jsons.size() < 3 || schedule_cache.get().day_jsons[0].empty()) {
			res.code = 500;
			res.content = "{\"data\":null,\"error\":\"Could not retrieve schedule.\"}";
		} else {
			res.code = 200;
			res.content = "{\"data\":{\"harmonogram\":" + schedule_cache.get().day_jsons[0] + ","
				"\"last_updated\":" + std::to_string(schedule_cache.get_last_update_time_since_epoch()) + "},\"error\":null}";
		}
		if (schedule_cache.should_update(std::chrono::minutes(30))) {
			fetch_tasks.emplace(fetch_task_t::SCHEDULE);
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
	fetch_tasks.push(fetch_task_t::SCHEDULE);
	while (true) {
		io_service.poll();
		if (fetcher.is_done()) {
			switch (current_fetch_task) {
			case fetch_task_t::NONE: break;
			case fetch_task_t::SCHEDULE:{
				if (fetcher.is_success()) {
					schedule_cache.update(api_server::schedule(fetcher.get_sheet()));
					std::cout << "\x1b[1m\x1b[96m[tasks::fetcher]: fetched schedule\x1b[0m" << std::endl;
				} else {
					std::cout << "\x1b[1m\x1b[91m[tasks::fetcher::error]: failed to fetch schedule\x1b[0m" << std::endl;
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
				case fetch_task_t::SCHEDULE:
					if (!schedule_cache.should_update(std::chrono::seconds(10))) {
						current_fetch_task = fetch_task_t::NONE;
						goto skip_fetch_task_assign;
					}
					if (!sheet.empty())
						fetcher.start_fetch(sheet, "harmonogramnaweb");
					break;
				default: break;
				}
				std::cout << "[tasks::fetcher]: assigned fetcher task " << fetch_task_strs[static_cast<size_t>(current_fetch_task)] << std::endl;
			}
		skip_fetch_task_assign:;
		}
		std::this_thread::sleep_for(std::chrono::milliseconds(50));
	}
	return 0;
}
