#ifndef HELPERS_H_INCLUDED
#define HELPERS_H_INCLUDED

#include <vector>

#include "polygon.h"

inline double is_left(const double p0x,
                      const double p0y,
                      const double p1x,
                      const double p1y,
                      const double p2x,
                      const double p2y);

int winding_number(const double px,
                   const double py,
                   std::vector<point> const &v);

#endif /* HELPERS_H_INCLUDED */
