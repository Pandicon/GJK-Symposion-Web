#ifndef __S2022_HTTP_HPP__
#define __S2022_HTTP_HPP__

#include <optional>
#include <string>
#include <unordered_map>

namespace api_server {
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
		static std::optional<http_response> parse_header(const std::string &header, size_t &content_length, bool &chunked);
	};
}

#endif
