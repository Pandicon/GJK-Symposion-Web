#ifndef __S2022_CONFIG_HPP__
#define __S2022_CONFIG_HPP__

#include <fstream>
#include <string>
#include <unordered_map>

namespace api_server {
	inline std::unordered_map<std::string, std::string> configs;
	inline void load_cfg(const std::string &file) {
		std::ifstream f(file);
		std::string buff;
		while (std::getline(f, buff)) {
			auto split = buff.find('=');
			if (split != std::string::npos) {
				configs[buff.substr(0, split)] = buff.substr(split + 1);
			}
		}
	}
	inline std::string get_config_or(const std::string &cfg, const std::string &default_val) {
		auto it = configs.find(cfg);
		if (it == configs.end()) return default_val;
		return it->second;
	}
}

#endif
