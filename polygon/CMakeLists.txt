include_directories(
    ${PROJECT_BINARY_DIR}/include
    )

add_library(
    polygon
    SHARED
    polygon.cpp
    polygon.h
    helpers.cpp
    helpers.h
    )

# clang seems to need this
target_link_libraries(
    polygon
    stdc++
    )

install(TARGETS polygon LIBRARY DESTINATION "lib")
install(FILES polygon.h DESTINATION "include")
install(FILES ${PROJECT_BINARY_DIR}/include/polygon_export.h DESTINATION "include")
