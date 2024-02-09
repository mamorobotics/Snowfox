# CMakeList.txt : CMake project for Snowfox, include source and define
# project specific logic here.
#

# Add source to this project's executable.
add_executable (Snowfox "Snowfox.cpp"  "Connection.cpp" "Connection.hpp" "ControllerValues.hpp" "ConnDetails.hpp" "Management.hpp" )


if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET Snowfox PROPERTY CXX_STANDARD 20)
endif()

set(LIB_DIR "${CMAKE_CURRENT_SOURCE_DIR}/lib")
message( ${LIB_DIR} )

# asio

include_directories("${LIB_DIR}/asio/include")

# jpeg-compressor

set(JPEG_DIR "${LIB_DIR}/jpeg-compressor")

add_subdirectory("${JPEG_DIR}")

target_link_libraries(${PROJECT_NAME} "jpge_lib")