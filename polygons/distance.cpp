// Copyright (C) 2004-2012 Per-Olof Persson

#include "distance.h"
#include <algorithm>
#include <math.h>

double dsegment(const double x0,
                const double y0,
                const double p1x,
                const double p1y,
                const double p2x,
                const double p2y)
{
    double v[2] = {p2x - p1x, p2y - p1y};
    double w[2] = {x0 - p1x, y0 - p1y};

    double c1 = v[0] * w[0] + v[1] * w[1];

    if (c1 <= 0.0)
    {
        return distance_squared(x0 - p1x, y0 - p1y);
    }

    double c2 = v[0] * v[0] + v[1] * v[1];

    if (c1 >= c2)
    {
        return distance_squared(x0 - p2x, y0 - p2y);
    }
    else
    {
        return distance_squared(x0 - (p1x + c1 / c2 * v[0]),
                                y0 - (p1y + c1 / c2 * v[1]));
    }
}
