include_directories(
    ${PROJECT_BINARY_DIR}/include
    )

add_library(
    polygons
    SHARED
    distance.cpp
    distance.h
    edge.h
    intersection.cpp
    intersection.h
    node.cpp
    node.h
    point.h
    polygons.cpp
    polygons.h
    )

add_executable(
    debug.x
    debug.cpp
    )

target_link_libraries(
    debug.x
    polygons
    )

# clang seems to need this
target_link_libraries(
    polygons
    stdc++
    )

install(TARGETS polygons LIBRARY DESTINATION "lib")
install(FILES polygons.h DESTINATION "include")
install(FILES ${PROJECT_BINARY_DIR}/include/polygons_export.h DESTINATION "include")
