#include "wserver.hpp"

#include <algorithm>
#include <iostream>
#include <sstream>
#include "config.hpp"

namespace api_server {
	wsession::wsession(asio::io_service &io_service,
#ifdef USE_SSL
		asio::ssl::context &ssl_ctx,
#endif
		request_callback_fun cb) : closed(false), socket(io_service
#ifdef USE_SSL
		,ssl_ctx
#endif
		), req_cb(cb) { }
	void wsession::start() {
#ifdef USE_SSL
		socket.async_handshake(asio::ssl::stream_base::server, [this](const asio::error_code &ec) {
			if (ec) {
				std::cerr << "[asio::error]: async handshake - " << ec.value() << " " << ec.message() << std::endl;
				closed = true;
				socket.close();
			} else {
				std::cout << "[handshake]: handshake successful - " << socket.lowest_layer().remote_endpoint().address() << std::endl;
#endif
				socket.async_read_some(asio::buffer(buff, buffer_size), [this](const asio::error_code &ec, size_t transferred_len) {
					if (ec) {
						std::cerr << "[asio::error]: async read some in async hanshake - " << ec.value() << " " << ec.message() << std::endl;
						closed = true;
						socket.close();
					} else {
						std::cout << "[read]: received a request - " << socket.lowest_layer().remote_endpoint().address() << std::endl;
						auto request = parse_http_request(transferred_len);
						if (request.has_value()) {
							std::cout << "[api]: valid request from " << socket.lowest_layer().remote_endpoint().address() << " " <<
								"method " << request->method << " url " << request->url << std::endl;
							http_response response = req_cb(request.value());
							std::cout << "[api]: response for " << socket.lowest_layer().remote_endpoint().address() << " " <<
								"content [" << response.content << "]" << std::endl;
							response.close = true;
							reply(response);
							socket.close();
						} else {
							std::cout << "[read]: invalid request - " << socket.lowest_layer().remote_endpoint().address() << std::endl;
							socket.close();
						}
					}
				});
#ifdef USE_SSL
			}
		});
#endif
	}
	std::optional<http_request> wsession::parse_http_request(size_t transferred) {
#define error_(msg, repl) do {\
			http_response bad_res;\
			bad_res.close = true;\
			bad_res.code = 400;\
			bad_res.content_type = "text/json";\
			std::cerr << "[parse_http_request::error]: " msg << std::endl;\
			bad_res.content = "{\"error\":\"" repl "\",\"data\":null}";\
			reply(bad_res);\
			return std::nullopt;\
		} while(0)
		if (transferred >= buffer_size) {
			error_("too long request!", "The request was too long!");
		}
		if (transferred < 6) {
			error_("too short request!", "The request was too short!");
		}
		http_request out;
		std::string sb(buff, transferred);
		std::string header;
		auto nl = sb.find("\r\n");
		if (nl == std::string::npos) { error_("invalid request.", "Invalid request."); }
		header = sb.substr(0, nl);
		sb = sb.substr(nl + 2), nl = sb.find("\r\n");
		for (; nl != std::string::npos; sb = sb.substr(nl + 2), nl = sb.find("\r\n")) {
			std::string line = sb.substr(0, nl);
			auto colon = line.find(": ");
			if (colon == std::string::npos) {
				if (line.size() == 0) {
					out.content = sb.substr(nl + 2);
					break;
				} else {
					error_("invalid request - header field line without colon.", "Invalid request (header field line without colon).");
				}
			} else {
				out.fields[line.substr(0, colon)] = line.substr(colon + 2);
			}
		}
		switch (header[0]) {
		case 'G':
			if (header.starts_with("GET")) { out.method = http_request::GET; header = header.substr(3); }
			else { error_("invalid request method.", "Invalid request method."); }
			break;
		case 'H':
			if (header.starts_with("HEAD")) { out.method = http_request::HEAD; header = header.substr(4); }
			else { error_("invalid request method.", "Invalid request method."); }
			break;
		case 'P':
			if (header.starts_with("POST")) { out.method = http_request::POST; header = header.substr(4); }
			else if (header.starts_with("PUT")) { out.method = http_request::PUT; header = header.substr(3); }
			else if (header.starts_with("PATCH")) { out.method = http_request::HEAD; header = header.substr(5); }
			else { error_("invalid request method.", "Invalid request method."); }
			break;
		case 'D':
			if (header.starts_with("DELETE")) { out.method = http_request::DELETE_M; header = header.substr(6); }
			else { error_("invalid request method.", "Invalid request method."); }
			break;
		case 'C':
			if (header.starts_with("CONNECT")) { out.method = http_request::CONNECT; header = header.substr(7); }
			else { error_("invalid request method.", "Invalid request method."); }
			break;
		case 'O':
			if (header.starts_with("OPTIONS")) { out.method = http_request::OPTIONS; header = header.substr(7); }
			else { error_("invalid request method.", "Invalid request method."); }
			break;
		case 'T':
			if (header.starts_with("TRACE")) { out.method = http_request::TRACE; header = header.substr(5); }
			else { error_("invalid request method.", "Invalid request method."); }
			break;
		default:
			error_("invalid request method.", "Invalid request method.");
		}
		if (header.empty()) { error_("empty request.", "Empty request."); }
		if (header[0] != ' ') { error_("invalid request.", "Invalid request."); }
		header = header.substr(1);
		auto url_end = header.find(' ');
		if (url_end == std::string::npos) { error_("invalid request.", "Invalid request."); }
		out.url = header.substr(0, url_end);
		// throw away the rest (HTTP version) :D
		return out;
#undef error_
	}
	void wsession::reply(const http_response &r) {
		reply_buff = r.encode();
		asio::async_write(socket, asio::buffer(reply_buff), [this, &r](const asio::error_code &ec, size_t transferred_len) {
			(void)transferred_len;
			if (ec) {
				std::cerr << "[asio::error]: async write in reply - " << ec.value() << " " << ec.message() << std::endl;
				closed = true;
			} else {
				//if (r.close)
					closed = true;
			}
		});
	}
	wserver::wserver(asio::io_service &io_service, unsigned short port, request_callback_fun cb) :
		io_service(io_service),
#ifdef USE_SSL
		ssl_ctx(asio::ssl::context::tls),
#endif
		req_cb(cb) {
#ifdef USE_SSL
		ssl_ctx.set_options(
			asio::ssl::context::default_workarounds |
			asio::ssl::context::no_sslv2 |
			asio::ssl::context::no_sslv3);
		ssl_ctx.use_certificate_chain_file(get_config_or("cert_chain_file", "cert.pem"));
		ssl_ctx.use_private_key_file(get_config_or("private_key_file", "private.pem"), asio::ssl::context::pem);
#endif
		bool ipv6 = get_config_or("use_ipv6", "") == "true";
		std::cout << "[info]: starting server on port " << port << ", IPv" << (ipv6 ? '6' : '4') << std::endl;
		acceptor = std::make_unique<asio::ip::tcp::acceptor>(io_service, asio::ip::tcp::endpoint(ipv6 ? asio::ip::tcp::v6() : asio::ip::tcp::v4(), port));
		wsession *next = new wsession(io_service, 
#ifdef USE_SSL
			ssl_ctx,
#endif
			req_cb);
		active_sessions.emplace_back(next);
		acceptor->async_accept(next->socket.lowest_layer(), [this, next](const asio::error_code &ec) {
			accept(ec, next);
		});
	}
	void wserver::accept(const asio::error_code &ec, wsession *session) {
		wsession *next = new wsession(io_service,
#ifdef USE_SSL
			ssl_ctx,
#endif
			req_cb);
		active_sessions.push_back(std::unique_ptr<wsession>(next));
		acceptor->async_accept(next->socket.lowest_layer(), [this, next](const asio::error_code &ec) {
			accept(ec, next);
		});
		filter_sessions();
		if (ec) {
			std::cerr << "[asio::error]: async accept - " << ec.value() << " " << ec.message() << std::endl;
		} else {
			session->start();
		}
	}
	void wserver::filter_sessions() {
		active_sessions.erase(std::remove_if(active_sessions.begin(), active_sessions.end(),
			[](const std::unique_ptr<api_server::wsession> &session) { return session->closed; }));
	}
}
