#include <stdio.h>
#include "polygons.h"

int main()
{
    polygons_context *context = polygons_new_context();

    double x[5] = {0.0, 1.0, 1.0, 0.0, 0.0};
    double y[5] = {0.0, 0.0, 1.0, 1.0, 0.0};

    polygons_add_polygon(context, 5, x, y);

    for (int i = 0; i < 5; i++)
    {
        x[i] += 5.0;
    }
    polygons_add_polygon(context, 5, x, y);

    for (int i = 0; i < 5; i++)
    {
        x[i] += 5.0;
    }
    polygons_add_polygon(context, 5, x, y);

    double px[2] = {0.0, 100.0};
    double py[2] = {0.0, 100.0};
    double distances[2];

    polygons_get_distances_to_nearest_edge(context, 2, px, py, distances);
    printf("distances:\n");
    for (int i = 0; i < 2; i++)
    {
        printf("%f\n", distances[i]);
    }

    polygons_free_context(context);

    return 0;
}
