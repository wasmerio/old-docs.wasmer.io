# Minimum cmake version required
cmake_minimum_required (VERSION 2.6)
# Name of our project
project (WasmerCHelloWorld)

# Output executable, and Entrypoint into our project
add_executable(hello-world hello-world.c)

# Added the Wasmer Runtime C API As a dependency
# Will be downloaded and compiled with rust
include(ExternalProject)
# set_directory_properties(PROPERTIES EP_PREFIX ${CMAKE_BINARY_DIR}/wasmer-build)
ExternalProject_Add(
        wasmer-runtime-c-api
        DOWNLOAD_DIR ${CMAKE_CURRENT_SOURCE_DIR}/cmake-build
        GIT_REPOSITORY https://github.com/wasmerio/wasmer.git
        GIT_TAG origin/master
        CONFIGURE_COMMAND ""
        BUILD_COMMAND cargo build -p wasmer-runtime-c-api
        COMMAND cargo build -p wasmer-runtime-c-api
        BINARY_DIR "${CMAKE_SOURCE_DIR}/dist/wasmer-build/src/wasmer-runtime-c-api/"
        INSTALL_COMMAND ""
        LOG_BUILD ON)

# Add the dependency to our hello-world executable we are ourputing
add_dependencies(hello-world wasmer-runtime-c-api)

# Handled some windows specific building
if(WIN32)
    set(WASMER_LIB "${CMAKE_SOURCE_DIR}/dist/wasmer-build/src/wasmer-runtime-c-api/target/debug/wasmer_runtime_c_api.dll")
else()
    set(WASMER_LIB "${CMAKE_SOURCE_DIR}/dist/wasmer-build/src/wasmer-runtime-c-api/target/debug/libwasmer_runtime_c_api${CMAKE_SHARED_LIBRARY_SUFFIX}")
endif()

# Error if Wasmer is not found
if(NOT WASMER_LIB)
    message(FATAL_ERROR "wasmer library not found")
endif()

# Link our hello world executable to the Wasmer runtime.
target_link_libraries(hello-world general ${WASMER_LIB})

