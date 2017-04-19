// This code is based on http://geomalgorithms.com/a03-_inclusion.html
// which is distributed under the following license:

// Copyright 2000 softSurfer, 2012 Dan Sunday
// This code may be freely used and modified for any purpose
// providing that this copyright notice is included with it.
// SoftSurfer makes no warranty for this code, and cannot be held
// liable for any real or imagined damage resulting from its use.
// Users of this code must verify correctness for their application.

#include "intersection.h"

// tests if a point is left|on|right of an infinite line
// input:  three points p0, p1, and p2
// return: > 0 for p2 left of the line through p0 and p1
//         = 0 for p2 on the line
//         < 0 for p2 right of the line
// FIXME inline
double is_left(const double p0x,
               const double p0y,
               const double p1x,
               const double p1y,
               const double p2x,
               const double p2y)
{
    return ((p1x - p0x) * (p2y - p0y) - (p2x - p0x) * (p1y - p0y));
}

// winding number test for a point in a polygon
// input:  p = a point,
//         v[] = vertex points of a polygon v[n+1] with v[n]=v[0]
// return: wn = the winding number (=0 only when p is outside)
int winding_number(const double px,
                   const double py,
                   std::vector<point> const &v)
{
    int wn = 0;

    // loop through all edges of the polygon
    for (int i = 0; i < (v.size() - 1); i++)
    {
        // edge from v[i] to  v[i+1]
        if (v[i].y <= py)
        {
            if (v[i + 1].y > py)
            {
                // an upward crossing
                if (is_left(v[i].x, v[i].y, v[i + 1].x, v[i + 1].y, px, py) >
                    0.0)
                {
                    // p left of edge
                    ++wn; // have a valid up intersect
                }
            }
        }
        else
        {
            if (v[i + 1].y <= py)
            {
                // a downward crossing
                if (is_left(v[i].x, v[i].y, v[i + 1].x, v[i + 1].y, px, py) <
                    0.0)
                {
                    // p right of edge
                    --wn; // have a valid down intersect
                }
            }
        }
    }

    return wn;
}
