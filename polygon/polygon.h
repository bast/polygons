#ifndef POLYGON_H_INCLUDED
#define POLYGON_H_INCLUDED

#ifndef POLYGON_API
#include "polygon_export.h"
#define POLYGON_API polygon_EXPORT
#endif

#ifdef __cplusplus
#include <vector>
#include <array>

struct point
{
    double x;
    double y;
};

class polygon_context
{
  public:
    polygon_context();
    ~polygon_context();

    void add_polygon(const int num_points, const double x[], const double y[]);
    void contains_points(const int num_points,
                         const double x[],
                         const double y[],
                         bool contains_points[]) const;

  private:
    polygon_context(const polygon_context &rhs);            // not implemented
    polygon_context &operator=(const polygon_context &rhs); // not implemented

    int num_polygons;
    std::vector<std::array<point, 2> > bounding_box;
    std::vector<std::vector<point> > polygons_v;

    // FIXME
    void check_that_context_is_initialized() const;
    bool is_initialized = false;
};
#endif

#ifdef __cplusplus
extern "C" {
#endif

#ifndef __cplusplus
struct polygon_context_s;
typedef struct polygon_context_s polygon_context;
#endif

POLYGON_API
polygon_context *polygon_new_context();

POLYGON_API
void polygon_free_context(polygon_context *context);

POLYGON_API
void polygon_add_polygon(polygon_context *context,
                        const int num_points,
                        const double x[],
                        const double y[]);

POLYGON_API
void polygon_contains_points(const polygon_context *context,
                            const int num_points,
                            const double x[],
                            const double y[],
                            bool contains_points[]);

#ifdef __cplusplus
}
#endif

#endif /* POLYGON_H_INCLUDED */
