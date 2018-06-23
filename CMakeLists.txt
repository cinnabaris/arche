cmake_minimum_required(VERSION 3.11)
project(arche)
set(ARCHE_VERSION_MAJOR 1)
set(ARCHE_VERSION_MINOR 0)
set(ARCHE_VERSION_PATCH 0)
set(CMAKE_CXX_STANDARD 17)

set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -O2")

aux_source_directory(src DIR_SRCS)
add_executable(arche ${DIR_SRCS})
target_link_libraries(arche pq)
