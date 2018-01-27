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

add_library(
    custom_functions
    SHARED
    custom_functions.cpp
    custom_functions.h
    )

# clang seems to need this
target_link_libraries(
    polygons
    stdc++
    custom_functions
    )

install(TARGETS polygons LIBRARY DESTINATION "lib")
install(FILES polygons.h DESTINATION "include")
install(FILES ${PROJECT_BINARY_DIR}/include/polygons_export.h DESTINATION "include")
