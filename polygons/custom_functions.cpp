#include "custom_functions.h"

double h_function(const double a) { return a; }

double g_function(const double distance)
{
    double scale_factor = 0.995792;
    return scale_factor * distance;
}
