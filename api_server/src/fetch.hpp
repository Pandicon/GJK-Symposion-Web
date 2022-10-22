#ifndef __S2022_FETCH_HPP__
#define __S2022_FETCH_HPP__

#include <memory>
#include <string>
#include <vector>
#include "../include/asio/ssl.hpp"
#include "../include/asio.hpp"
#include "http.hpp"

namespace api_server {
	constexpr unsigned int fetcher_buffer_size = 2048;
	using sheet_t = std::vector<std::vector<std::string>>;
	using ssl_socket_t = asio::ssl::stream<asio::ip::tcp::socket>;

	class sheet_fetcher {
	public:
		sheet_fetcher(asio::io_service &io_service);
		inline bool is_done() const { return done; }
		inline bool is_success() const { return success; }
		inline const sheet_t &get_sheet() const { return data; }
		void start_fetch(const std::string &sheet, const std::string &list);
	private:
		bool done, success;
		sheet_t data;
		asio::io_service &io_service;
		asio::ssl::context ssl_ctx;
		asio::ip::tcp::resolver resolver;
		std::unique_ptr<ssl_socket_t> socket;
		std::string buff;
		char recv_buff[fetcher_buffer_size];

		void receive();
	};
}

#endif
