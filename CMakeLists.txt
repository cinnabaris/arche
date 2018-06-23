cmake_minimum_required(VERSION 3.11)
project(arche)

set(ARCHE_VERSION_MAJOR 1)
set(ARCHE_VERSION_MINOR 0)
set(ARCHE_VERSION_PATCH 0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -O2 -DNDEBUG")

include_directories(${CMAKE_CURRENT_LIST_DIR}/src)
aux_source_directory(${CMAKE_CURRENT_LIST_DIR}/src DIR_SRC)
aux_source_directory(${CMAKE_CURRENT_LIST_DIR}/src/plugins DIR_SRC_PLUGINS)
aux_source_directory(${CMAKE_CURRENT_LIST_DIR}/src/plugins/ops DIR_SRC_PLUGINS_OPS)

add_executable(${PROJECT_NAME} ${DIR_SRC} ${DIR_SRC_PLUGINS} ${DIR_SRC_PLUGINS_OPS})
target_link_libraries(
  ${PROJECT_NAME}
  pq odbc odbccr odbcinst
  rabbitmq
  PocoCrypto PocoData PocoDataODBC PocoEncodings PocoFoundation PocoJSON PocoNet PocoRedis PocoUtil PocoXML PocoZip
)
