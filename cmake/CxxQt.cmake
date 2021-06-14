# TODO: does minimum version need to be set in the module as well?
# TODO: have further parameters for different options and folders etc
# TODO: will all builds want an executable? this might need to be separate
#
# APP_NAME is used as the executable name and the prefix for the lib name
# RUST_SOURCES is the list of rust source files that lead to generated C++ files
# CPP_SOURCES are C++ files not generated by build.rs that we want to compile
function(cxx_qt_cmake APP_NAME RUST_SOURCES CPP_SOURCES)
    cxx_qt_generate_cpp(${RUST_SOURCES} GEN_SOURCES)

    # And specify that we want CMake to build these sources
    add_executable(${APP_NAME} ${CPP_SOURCES} ${GEN_SOURCES})

    cxx_qt_include(${APP_NAME})

    cxx_qt_link_rustlib(${APP_NAME})
endfunction()

# Generate the C++ sources for the given rust sources
#
# RUST_SOURCES is the list of rust source files that lead to generated C++ files
# GEN_SOURCES an output variable that generated C++ files are listed into
function(cxx_qt_generate_cpp RUST_SOURCES GEN_SOURCES)
    # TODO: figure out if RelWithDebInfo is a thing in Rust and fix accordingly
    if (CMAKE_BUILD_TYPE STREQUAL "Debug")
        set(CARGO_CMD cargo build)
        set(TARGET_DIR "debug")
    else ()
        set(CARGO_CMD cargo build --release)
        set(TARGET_DIR "release")
    endif ()

    # We list the rust source files that lead to generated C++ files here
    # so that CMake is forced to re-run cargo and parse the list it produces
    # during the config stage when this list of source files changes.
    file(MAKE_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/target/cxx-qt-gen")
    file(WRITE "${CMAKE_CURRENT_SOURCE_DIR}/target/cxx-qt-gen/rust_sources.txt" "${RUST_SOURCES}")

    # Run cargo during config to ensure the cpp source file list is created
    execute_process(
        COMMAND ${CARGO_CMD}
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    )

    # Now we can read the list of C++ files that cargo produced
    file(STRINGS "${CMAKE_CURRENT_SOURCE_DIR}/target/cxx-qt-gen/cpp_sources.txt" CPP_GEN_SOURCES)

    # Pass the generated sources back
    set(${GEN_SOURCES} ${CPP_GEN_SOURCES} PARENT_SCOPE)
endfunction()

# Set the target include dirs of the C++ executable / library to include the rust / cxx dirs
#
# APP_NAME is the executable / library name which include dirs are being set to
function(cxx_qt_include APP_NAME)
    # FIXME: should this be target/cxx-qt-gen/include?
    target_include_directories(${APP_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")
    # FIXME: should this be target/cxx-qt-gen/src ?
    target_include_directories(${APP_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/target")
    target_include_directories(${APP_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/target/cxx-qt-gen/statics")
endfunction()

# Link the generated rust library with the C++ executable / library
#
# APP_NAME is the executable / library name which the rustlib is a dependency of
function(cxx_qt_link_rustlib APP_NAME)
    # Find the threads package for the system
    set(CMAKE_THREAD_PREFER_PTHREAD TRUE)
    find_package(Threads REQUIRED)

    # TODO: figure out if RelWithDebInfo is a thing in Rust and fix accordingly
    if (CMAKE_BUILD_TYPE STREQUAL "Debug")
        set(CARGO_CMD cargo build)
        set(TARGET_DIR "debug")
    else ()
        set(CARGO_CMD cargo build --release)
        set(TARGET_DIR "release")
    endif ()

    # We also list the .a produced by cargo as a dependency so that cargo gets a
    # chance to rebuild the .a every time that a cmake build is run.
    # TODO: use correct binary name on windows
    set(RUST_PART_LIB "${CMAKE_CURRENT_SOURCE_DIR}/target/${TARGET_DIR}/librust.a")
    add_custom_target(
        "${APP_NAME}_rustlib"
        COMMAND ${CARGO_CMD}
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    )
    add_dependencies(${APP_NAME} "${APP_NAME}_rustlib")

    # The Rust lib also needs to be linked to pthread and dl
    # TODO: figure out the equivalent on windows
    target_link_libraries(${APP_NAME} ${RUST_PART_LIB} Threads::Threads ${CMAKE_DL_LIBS})
endfunction()