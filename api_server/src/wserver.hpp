#ifndef __S2022_WSERVER_HPP__
#define __S2022_WSERVER_HPP__

#include <memory>
#include <optional>
#include <string>
#include <unordered_map>
#include <vector>
#include "../include/asio.hpp"
#include "../include/asio/ssl.hpp"
#include "http.hpp"

namespace api_server {
	constexpr unsigned int buffer_size = 2048;
	
	using request_callback_fun = http_response (*)(const http_request &);
	using ssl_socket = 
#ifdef USE_SSL
		asio::ssl::stream<
#endif
		asio::ip::tcp::socket
#ifdef USE_SSL
		>
#endif
		;
	class wsession {
	public:
		bool closed;
		ssl_socket socket;

		wsession(asio::io_service &io_service, 
#ifdef USE_SSL
			asio::ssl::context &ssl_ctx,
#endif
			request_callback_fun cb);
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
#ifdef USE_SSL
		asio::ssl::context ssl_ctx;
#endif
		request_callback_fun req_cb;

		void filter_sessions();
		void accept(const asio::error_code &ec, wsession *session);
	};
}

#endif
