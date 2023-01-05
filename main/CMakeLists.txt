

idf_component_register(SRCS "esp32-project-with-rust-component.c"
                    INCLUDE_DIRS ".")

set(RUST_STATIC_LIBRARY "${CMAKE_CURRENT_SOURCE_DIR}/../components/example_rust/xtensa-esp32-none-elf/release/libexample_rust.rlib")

target_link_libraries(${COMPONENT_LIB} ${RUST_STATIC_LIBRARY})