#pragma once

#ifdef __cplusplus
extern "C" {
#endif

static inline double distance_squared(double x, double y)
{
    //  return sqrt(x*x + y*y);
    return x * x + y * y; // we cheat - we compute square root at the very end
                          // only for the closest line
                          //            instead of every line
}

double dsegment(const double x0,
                const double y0,
                const double p1x,
                const double p1y,
                const double p2x,
                const double p2y);

#ifdef __cplusplus
}
#endif
