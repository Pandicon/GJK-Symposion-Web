#include <chrono>
#include <iostream>
#include <sstream>
#include <thread>
#include "./config.hpp"
#include "./wserver.hpp"

api_server::http_response reqest_callback(const api_server::http_request &req) {
	api_server::http_response res;
	res.close = true;
	res.code = 404;
	res.content_type = "text/json";
	res.content = "{\"error\":\"Unknown url.\",\"data\":null}";
	if (req.method != api_server::http_request::GET) {
		res.code = 403;
		res.content = "{\"error\":\"Any request method other than GET isn't allowed.\",\"data\":null}";
	} else if (req.url == "/secret") {
		res.code = 200;
		res.content = "{\"data\":{\"secret_message\":\"Nazdárek!\"},\"error\":null}";
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
	while (true) {
		io_service.run();
		std::this_thread::sleep_for(std::chrono::milliseconds(50));
	}
	return 0;
}
