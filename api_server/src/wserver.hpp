#ifndef __S2022_WSERVER_HPP__
#define __S2022_WSERVER_HPP__

#include <memory>
#include <optional>
#include <string>
#include <unordered_map>
#include <vector>
#include "../include/asio.hpp"
#include "../include/asio/ssl.hpp"

namespace api_server {
	constexpr unsigned int buffer_size = 1024;

	struct http_request {
	public:
		enum method_t {
			GET, HEAD, POST, PUT, DELETE, CONNECT, OPTIONS, TRACE, PATCH
		};

		method_t method;
		std::string url;
		std::unordered_map<std::string, std::string> fields;
		std::string content;
	};
	struct http_response {
	public:
		unsigned short code;
		std::string content;
		std::string content_type;
		bool close;

		std::string encode() const;
	};
	using request_callback_fun = http_response (*)(const http_request &);
	using ssl_socket = asio::ssl::stream<asio::ip::tcp::socket>;
	class wsession {
	public:
		bool closed;
		ssl_socket socket;

		wsession(asio::io_service &io_service, asio::ssl::context &ssl_ctx, request_callback_fun cb);
		void start();
		std::optional<http_request> parse_http_request(size_t transferred);
		void reply(const http_response &r);
	private:
		request_callback_fun req_cb;
		char buff[buffer_size];
		std::string reply_buff;
	};
	class wserver {
	public:
		wserver(asio::io_service &io_service, unsigned short port, request_callback_fun cb);
	private:
		std::vector<std::unique_ptr<wsession>> active_sessions;
		asio::io_service &io_service;
		std::unique_ptr<asio::ip::tcp::acceptor> acceptor;
		asio::ssl::context ssl_ctx;
		request_callback_fun req_cb;

		void filter_sessions();
		void accept(const asio::error_code &ec, wsession *session);
	};
}

#endif
