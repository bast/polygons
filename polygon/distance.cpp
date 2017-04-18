// Copyright (C) 2004-2012 Per-Olof Persson. See COPYING.TXT for details.

#include <algorithm>
#include <math.h>
#include "distance.h"
#include <limits>


// Quick routines
static inline double length(double x, double y)
{
//  return sqrt(x*x + y*y);
    return      x*x + y*y;  // we cheat - we compute square root at the very end only for the closest line
                            //            instead of every line
}


double dsegment(const double x0,
                const double y0,
                const double p1x,
                const double p1y,
                const double p2x,
                const double p2y)
{
  double v[2] = {p2x - p1x, p2y - p1y};
  double w[2] = {x0 - p1x, y0 - p1y};

  double c1 = v[0]*w[0] + v[1]*w[1];

  if (c1 <= 0.0)
  {
    return length(x0 - p1x, y0 - p1y);
  }

  double c2 = v[0]*v[0] + v[1]*v[1];

  if (c1 >= c2)
  {
    return length(x0 - p2x, y0 - p2y);
  }
  else
  {
    return length(x0 - (p1x + c1/c2*v[0]),
                  y0 - (p1y + c1/c2*v[1]));
  }
}


double vdsegment(const int num_points,
                 const double ps_x[],
                 const double ps_y[],
                 const int num_vertices,
                 const double vs_x[],
                 const double vs_y[],
                       double distances[])
{
    double huge = std::numeric_limits<float>::max();

    for (int ip = 0; ip < num_points; ip++)
    {
        double d = huge;
        for (int iv = 0; iv < num_vertices - 1; iv++)
        {
            double _d = dsegment(ps_x[ip], ps_y[ip], vs_x[iv], vs_y[iv], vs_x[iv+1], vs_y[iv+1]);
            d = std::min(d, _d);
        }
        distances[ip] = sqrt(d);  // cheat, see above sqrt cheat
    }
}
