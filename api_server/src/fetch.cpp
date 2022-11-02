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
					constexpr size_t limit = 4096;
					char arr[limit];
					buff.clear();
					while (true) {
						size_t nbytes = socket->read_some(asio::buffer(arr, limit), cec);
						if (cec) { std::cerr << "[sheet_fetcher]: unable to retrieve content " << cec.value() << " " << cec.message() << std::endl; done = true; return; }
						std::string arr_s(arr, nbytes);
						auto nl = arr_s.find("\r\n");
						if (nl == std::string::npos) { std::cerr << "[sheet_fetcher]: failed to fetch content" << std::endl; done = true; return; }
						//std::cout << "[sheet_fetcher]: chunked transfer - incoming first chunk " << (nbytes - nl - 2) << ", [" << arr_s.substr(nl + 2) << "]" << std::endl;
						buff.append(arr_s.substr(nl + 2));
						arr_s = arr_s.substr(0, nl);
						if (!arr_s.empty() && arr_s.find_first_not_of("0123456789ABCDEFabcdef") != std::string::npos) { std::cerr << "[sheet_fetcher]: invalid chunk size (" << arr_s << ")" << std::endl; done = true; return; }
						size_t to_recv = std::stoul(arr_s, 0, 16) + 2; // + 2 is \r\n
						if (to_recv < 3) {
							//std::cout << "[sheet_fetcher]: chunked transfer - end" << std::endl;
							break;
						}
						size_t recv = nbytes - nl - 2;
						for (; recv < to_recv; recv += nbytes) {
							nbytes = socket->read_some(asio::buffer(arr, limit), cec);
							if (cec) { std::cerr << "[sheet_fetcher]: unable to retrieve content " << cec.value() << " " << cec.message() << std::endl; done = true; return; }
							buff.append(arr, nbytes);
							//std::cout << "[sheet_fetcher]: chunked transfer - incoming " << nbytes << ", [" << std::string(arr, nbytes) << "]" << std::endl;
						}
						buff.pop_back(); // ignore the \r\n at the end of chunk
						buff.pop_back();
						//std::cout << "[sheet_fetcher]: chunk transfer finished with size " << recv << ", chunk size is " << to_recv << std::endl;
					}
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
				data.clear();
				data.shrink_to_fit();
				std::string b;
				bool in_text = false;
				bool prev_quotes = false;
				data.emplace_back();
				for (const char &c : buff) {
					if (c == '"') {
						if (!in_text && prev_quotes) {
							b.push_back(c);
						}
						prev_quotes = true;
						in_text = !in_text;
					} else if (!in_text && c == ',') {
						data.back().emplace_back(b);
						b.clear();
						prev_quotes = false;
					} else if (!in_text && c == '\n') {
						data.back().emplace_back(b);
						b.clear();
						data.emplace_back();
						prev_quotes = false;
					} else if (!in_text && c == '\r') {
						prev_quotes = false;
					} else {
						b.push_back(c);
						prev_quotes = false;
					}
				}
				data.pop_back();
				buff.clear();
				buff.shrink_to_fit();
				done = true;
				success = true;
			}
			if (success && !data.empty()) {
				std::cerr << "[sheet_fetcher]: fetched sheet - " << data.front().size() << "x" << data.size() << std::endl;
				for (const auto &i : data) {
					std::cerr << "[sheet_fetcher]:     ";
					for (const auto &j : i) {
						std::cerr << (j.empty() ? '.' : '#');
					}
					std::cerr << std::endl;
				}
			}
		});
	}
}
