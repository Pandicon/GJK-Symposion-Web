#include "fetch.hpp"

#include <algorithm>
#include <functional>
#include <iostream>

namespace api_server {
	sheet_fetcher::sheet_fetcher(asio::io_service &io_service) : done(true), success(false), io_service(io_service),
		ssl_ctx(asio::ssl::context_base::tls_client), resolver(io_service) { }
	void sheet_fetcher::start_fetch(const std::string &sheet, const std::string &list) {
		done = success = false;
		asio::error_code ec;
		auto endpoints = resolver.resolve("docs.google.com", "443", ec);
		if (ec) {
			std::cerr << "[sheet_fetcher]: unable to resolve docs.google.com:443 " << ec.value() << " " << ec.message() << std::endl;
			done = true;
			return;
		}
		socket = std::make_unique<ssl_socket_t>(io_service, ssl_ctx);
		asio::async_connect(socket->lowest_layer(), endpoints, [this, sheet, list](const asio::error_code &ec, const asio::ip::tcp::endpoint &ep) {
			std::cout << "[sheet_fetcher]: fetching from " << ep.address() << ":" << ep.port() << std::endl;
			if (ec) {
				std::cerr << "[sheet_fetcher]: unable to connect to docs.google.com:443 " << ec.value() << " " << ec.message() << std::endl;
				done = true;
			} else {
				socket->async_handshake(asio::ssl::stream_base::client, [this, sheet, list](const asio::error_code &ec) {
					if (ec) {
						std::cerr << "[sheet_fetcher]: handshake failed " << ec.value() << " " << ec.message() << std::endl;
						done = true;
					} else {
						buff = "GET /spreadsheets/d/" + sheet + "/gviz/tq?tqx=out:csv&sheet=" + list + " HTTP/1.1\r\n"
							"Host: docs.google.com\r\n"
							"user-agent: sfetch/1.0\r\n"
							"\r\n";
						socket->async_write_some(asio::buffer(buff), [this](const asio::error_code &ec, size_t transferred) {
							(void)transferred;
							if (ec) {
								std::cerr << "[sheet_fetcher]: unable to send request " << ec.value() << " " << ec.message() << std::endl;
								done = true;
							} else {
								std::cout << "[sheet_fetcher]: request:\n" << buff << std::endl;
								receive();
							}
						});
					}
				});
			}
		});
	}
	void sheet_fetcher::receive() {
		socket->async_read_some(asio::buffer(recv_buff, fetcher_buffer_size), [this](const asio::error_code &ec, size_t transferred) {
			if (ec) {
				std::cerr << "[sheet_fetcher]: unable to retrieve data " << ec.value() << " " << ec.message() << std::endl;
				done = true;
			} else {
				buff = std::string(recv_buff, transferred);
				auto it = buff.find("\r\n\r\n");
				if (it == std::string::npos) {
					std::cerr << "[sheet_fetcher]: invalid response (header end not found) " << ec.value() << " " << ec.message() << std::endl;
					done = true;
				}
				std::cout << "response: " << buff << std::endl;
				bool chunked = false;
				size_t content_length;
				auto res = http_response::parse_header(buff.substr(0, it + 2), content_length, chunked);
				if (!res.has_value()) {
					std::cerr << "[sheet_fetcher]: invalid response " << ec.value() << " " << ec.message() << std::endl;
					done = true;
					return;
				}
				buff = buff.substr(it + 4);
				if (chunked) {
					asio::error_code cec;
					// TODO: read the chunk size
					// I tried to do it, but I couldn't get it to work, so the data limit is 64KiB now
					constexpr size_t limit = 65536;
					char arr[limit];
					size_t nbytes = socket->read_some(asio::buffer(arr, limit), cec);
					if (cec) { std::cerr << "[sheet_fetcher]: unable to retrieve content " << cec.value() << " " << cec.message() << std::endl; done = true; return; }
					std::string tmp_str(arr, nbytes);
					auto nl = tmp_str.find("\r\n");
					if (nl == std::string::npos) {
						std::cerr << "[sheet_fetcher]: failed to fetch content" << std::endl;
						done = true; return;
					}
					buff = tmp_str.substr(nl + 2);
				} else {
					size_t curr_size = transferred - it - 4; // content size so far
					if (curr_size < content_length) {
						std::string content_buff(content_length - curr_size, '0');
						asio::error_code cec;
						socket->read_some(asio::buffer(content_buff), cec);
						if (cec) {
							std::cerr << "[sheet_fetcher]: unable to retrieve content " << cec.value() << " " << cec.message() << std::endl;
							done = true;
							return;
						}
						buff += content_buff;
					}
				}
				std::cerr << "[sheet_fetcher]: sheet data " << buff << std::endl;
				std::istringstream iss(buff);
				data.clear();
				data.shrink_to_fit();
				std::string ln;
				while (std::getline(iss, ln)) {
					if (!ln.empty()) {
						ln.erase(ln.begin(), std::find_if(ln.begin(), ln.end(), std::not1(std::ptr_fun<int, int>(std::isspace))));
						ln.erase(std::find_if(ln.rbegin(), ln.rend(), std::not1(std::ptr_fun<int, int>(std::isspace))).base(), ln.end());
						std::string cell;
						data.emplace_back();
						for (auto it = ln.find(','); it != std::string::npos; it = ln.find(',')) {
							cell = ln.substr(0, it);
							ln = ln.substr(it + 1);
							if (cell.empty()) continue;
							if (cell.size() > 1 && cell.front() == '"' && cell.back() == '"') {
								cell = cell.substr(1, cell.size() - 2);
							}
							//std::cout << "cell " << cell << std::endl;
							data.back().emplace_back(cell);
						}
						if (!ln.empty()) {
							if (ln.size() > 1 && ln.front() == '"' && ln.back() == '"') {
								ln = ln.substr(1, ln.size() - 2);
							}
							//std::cout << "cell " << ln << std::endl;
							data.back().emplace_back(ln);
						}
					}
				}
				buff.clear();
				buff.shrink_to_fit();
				done = true;
				success = true;
			}
		});;
	}
}
