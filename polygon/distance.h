#ifndef DISTANCE_H_INCLUDED
#define DISTANCE_H_INCLUDED

#ifdef __cplusplus
extern "C" {
#endif

double vdsegment(const int num_points,
                 const double ps_x[],
                 const double ps_y[],
                 const int num_vertices,
                 const double vs_x[],
                 const double vs_y[],
                       double distances[]);

#ifdef __cplusplus
}
#endif

#endif /* DISTANCE_H_INCLUDED */
