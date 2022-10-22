#ifndef __S2022_TABLE_HPP__
#define __S2022_TABLE_HPP__

#include <stdint.h>
#include <chrono>

namespace api_server {
	template<typename T>
	class cache {
	public:
		inline void update(const T &src) { last_update = std::chrono::system_clock::now(); cached = src; }
		inline T get() { return cached; }
		inline std::chrono::system_clock::time_point get_last_update() { return last_update; }
		inline uint64_t get_last_update_time_since_epoch() { return static_cast<uint64_t>(std::chrono::duration_cast<std::chrono::seconds>(last_update.time_since_epoch()).count()); }
		template<typename _Rep, typename _Period>
		inline bool should_update(const std::chrono::duration<_Rep, _Period> &max_age) { return std::chrono::system_clock::now() - last_update > max_age; }
	protected:
		std::chrono::system_clock::time_point last_update;
		T cached;
	};
	template<typename T>
	class json_cache : public cache<T> {
	public:
		inline void update(const T &src, const std::string &json) { this->last_update = std::chrono::system_clock::now(); this->cached = src; cached_json = json; }
		inline std::string json() { return cached_json; }
	private:
		std::string cached_json;
	};
}

#endif
