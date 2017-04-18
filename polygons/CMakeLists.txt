include_directories(
    ${PROJECT_BINARY_DIR}/include
    )

add_library(
    polygons
    SHARED
    polygons.cpp
    polygons.h
    intersection.cpp
    intersection.h
    distance.cpp
    distance.h
    )

# clang seems to need this
target_link_libraries(
    polygons
    stdc++
    )

install(TARGETS polygons LIBRARY DESTINATION "lib")
install(FILES polygons.h DESTINATION "include")
install(FILES ${PROJECT_BINARY_DIR}/include/polygons_export.h DESTINATION "include")
