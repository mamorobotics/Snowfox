# CMakeList.txt : CMake project for Snowfox, include source and define
# project specific logic here.
#

# Add source to this project's executable.
add_executable (Snowfox "Snowfox.cpp" "Snowfox.hpp")

if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET Snowfox PROPERTY CXX_STANDARD 20)
endif()

# TODO: Add tests and install targets if needed.

find_package(asio CONFIG REQUIRED)
target_link_libraries(Snowfox PRIVATE asio::asio)