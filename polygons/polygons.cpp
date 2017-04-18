#include <stdio.h>
#include <iostream>
#include <fstream>
#include <random>
#include <limits>

#include "polygons.h"
#include "intersection.h"
#include "distance.h"

#define AS_TYPE(Type, Obj) reinterpret_cast<Type *>(Obj)
#define AS_CTYPE(Type, Obj) reinterpret_cast<const Type *>(Obj)

void polygons_context::check_that_context_is_initialized() const
{
    if (not is_initialized)
    {
        fprintf(stderr, "ERROR: context is not initialized\n");
        exit(-1);
    }
}

POLYGONS_API
polygons_context *polygons_new_context()
{
    return AS_TYPE(polygons_context, new polygons_context());
}
polygons_context::polygons_context() { is_initialized = true; }

POLYGONS_API
void polygons_free_context(polygons_context *context)
{
    if (!context)
        return;
    delete AS_TYPE(polygons_context, context);
}
polygons_context::~polygons_context()
{
    nodes.clear();
    is_initialized = false;
}

std::vector<node> build_nodes(const int n, const std::vector<node> children)
{
    int num_new_nodes = children.size() / n;
    if (children.size() % n > 0)
        num_new_nodes++;

    std::vector<node> new_nodes;

    int i = 0;
    for (int k = 0; k < num_new_nodes; k++)
    {
        node new_node;
        for (int l = 0; l < n; l++)
        {
            if (i < children.size())
            {
                new_node.add_child_node(children[i]);
                i++;
            }
        }
        new_nodes.push_back(new_node);
    }

    return new_nodes;
}

POLYGONS_API
void polygons_add_polygon(polygons_context *context,
                          const int num_points,
                          const double x[],
                          const double y[])
{
    AS_TYPE(polygons_context, context)->add_polygon(num_points, x, y);
}
void polygons_context::add_polygon(const int num_points,
                                   const double x[],
                                   const double y[])
{
    check_that_context_is_initialized();

    int N = 4; // FIXME

    int num_nodes = num_points / N;
    if (num_points % N > 0)
        num_nodes++;

    int i = 0;
    for (int k = 0; k < num_nodes; k++)
    {
        node new_node;
        for (int l = 0; l < N; l++)
        {
            if (i < num_points)
            {
                edge e = {x[i], y[i]};
                new_node.add_child_edge(e);
                i++;
            }
        }
        nodes.push_back(new_node);
    }

    while (nodes.size() > 1)
    {
        // FIXME N is hardcoded to the same value as above
        nodes = build_nodes(N, nodes);
    }
}

POLYGONS_API
double polygons_get_distance(const polygons_context *context,
                             const double x,
                             const double y)
{
    return AS_CTYPE(polygons_context, context)->get_distance(x, y);
}
double polygons_context::get_distance(const double px, const double py) const
{
    double large_number = std::numeric_limits<double>::max();
    point p = {px, py};
    return nodes[0].get_distance(large_number, p);
}
