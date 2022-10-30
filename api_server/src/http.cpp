#include "http.hpp"

#include <cctype>
#include <algorithm>
#include <iostream>
#include <sstream>

namespace api_server {
	std::string http_response::encode() const {
		std::ostringstream oss;
		oss << "HTTP/1.0 " << code << " ";
		switch (code) {
		case 200: oss << "OK"; break;
		case 400: oss << "Bad Request"; break;
		case 401: oss << "Unauthorized"; break;
		case 403: oss << "Forbidden"; break;
		case 404: oss << "Not Found"; break;
		case 500: oss << "Internal Server Error"; break;
		default: oss << "Unknown code"; break;
		}
		oss << "\r\n";
		if (!content.empty()) {
			oss << "Content-Type: " << content_type << "\r\n";
			oss << "Content-Length: " << content.size() << "\r\n";
		}
		//if (close) {
		//	oss << "Connection: close\r\n";
		//}
		oss << "Access-Control-Allow-Origin: *\r\n";
		oss << "\r\n";
		if (!content.empty()) {
			oss << content;
		}
		return oss.str();
	}
	std::optional<http_response> http_response::parse_header(const std::string &header, size_t &content_length, bool &chunked) {
		if (header.size() < 6) {
			std::cerr << "[http_response::parse_header::error]: header too short!" << std::endl;
			return std::nullopt;
		}
		std::string s(header);
		http_response out;
		auto nl = s.find("\r\n");
		if (nl == std::string::npos) {
			std::cerr << "[http_response::parse_header::error]: header without newline." << std::endl;
			return std::nullopt;
		}
		std::string first_line(s.substr(0, nl));
		s = s.substr(nl + 2);
		for (nl = s.find("\r\n"); nl != std::string::npos; s = s.substr(nl + 2), nl = s.find("\r\n")) {
			std::string line = s.substr(0, nl);
			auto colon = line.find(": ");
			if (colon == std::string::npos) {
				std::cerr << "[http_response::parse_header::error]: header field line without colon." << std::endl;
				return std::nullopt;
			} else {
				std::string field(line.substr(0, colon));
				std::for_each(field.begin(), field.end(), [](char &c) { c = std::tolower(static_cast<unsigned char>(c)); });
				std::string value(line.substr(colon + 2));
				if (field == "content-type") { out.content_type = value; }
				else if (field == "content-length") {
					std::istringstream viss(value);
					viss >> content_length;
					if (viss.fail() || !viss.eof()) {
						std::cerr << "[http_response::parse_header::error]: content-length is not a number." << std::endl;
						return std::nullopt;
					}
				} else if (field == "transfer-encoding") { chunked = (value == "chunked"); }
			}
		}
		auto space = first_line.find(' ');
		if (space == std::string::npos) {
			std::cerr << "[http_response::parse_header::error]: space before HTTP code not found" << std::endl;
			return std::nullopt;
		}
		first_line = first_line.substr(space + 1);
		space = first_line.find(' ');
		if (space == std::string::npos) {
			std::cerr << "[http_response::parse_header::error]: space after HTTP code not found" << std::endl;
			return std::nullopt;
		}
		std::istringstream iss(first_line.substr(0, space));
		iss >> out.code;
		if (iss.fail() || !iss.eof()) {
			std::cerr << "[http_response::parse_header::error]: HTTP code wasn't a number" << std::endl;
			return std::nullopt;
		}
		return out;
	}
}
