cmake_minimum_required(VERSION 3.0.0)
project(Symposion2022_api_server VERSION 0.1.0)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin/)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED true)
set(CMAKE_CXX_WA)

include(CTest)
enable_testing()

add_executable(api_server ${CMAKE_SOURCE_DIR}/src/main.cpp ${CMAKE_SOURCE_DIR}/src/wserver.cpp ${CMAKE_SOURCE_DIR}/src/http.cpp
	${CMAKE_SOURCE_DIR}/src/fetch.cpp)
target_include_directories(api_server PUBLIC ${CMAKE_SOURCE_DIR}/include/)
if(MSVC)
	target_compile_options(api_server PRIVATE /W4 /WX)
else()
	target_compile_options(api_server PRIVATE -Wall -Wextra -Wpedantic -Werror)
endif()
target_link_libraries(api_server ssl crypto)
