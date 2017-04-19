#include <stdio.h>
#include "polygons.h"

int main()
{
    polygons_context *context = polygons_new_context();

    double x[30] = {10.0,
                    11.0,
                    12.0,
                    13.0,
                    14.0,
                    15.0,
                    16.0,
                    17.0,
                    18.0,
                    19.0,
                    20.0,
                    21.0,
                    22.0,
                    23.0,
                    24.0,
                    25.0,
                    26.0,
                    27.0,
                    28.0,
                    29.0,
                    30.0,
                    31.0,
                    32.0,
                    33.0,
                    34.0,
                    35.0,
                    36.0,
                    37.0,
                    38.0,
                    39.0};
    polygons_add_polygon(context, 30, x, x);

    for (int i = 0; i < 30; i++)
    {
        x[i] += 10.0;
    }
    polygons_add_polygon(context, 30, x, x);

    for (int i = 0; i < 30; i++)
    {
        x[i] += 10.0;
    }
    polygons_add_polygon(context, 30, x, x);

    double px[5] = {0.0, 1.0, 2.0, 3.0, 100.0};
    double py[5] = {0.0, 1.0, 2.0, 3.0, 100.0};
    double distances[5];

    polygons_get_distances(context, 5, px, py, distances);
    printf("distances:\n");
    for (int i = 0; i < 5; i++)
    {
        printf("%f\n", distances[i]);
    }

    polygons_free_context(context);

    return 0;
}
