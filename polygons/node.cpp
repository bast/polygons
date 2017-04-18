//#include <stdio.h>
//#include <iostream>
//#include <fstream>
//#include <random>
#include <limits>
//
//#include "polygons.h"
//#include "intersection.h"
#include "edge.h"
#include "node.h"
#include "distance.h"
//
//#define AS_TYPE(Type, Obj) reinterpret_cast<Type *>(Obj)
//#define AS_CTYPE(Type, Obj) reinterpret_cast<const Type *>(Obj)

// if best case distance is larger than currently optimum distance, this box is
// rejected
// the box is region 5:
//    |   |
//  1 | 2 | 3
// ___|___|___
//    |   |
//  4 | 5 | 6
// ___|___|___
//    |   |
//  7 | 8 | 9
//    |   |
bool skip_box(const double d,
              const point p,
              const double xmin,
              const double xmax,
              const double ymin,
              const double ymax)
{
    if (p.y > ymax)
    {
        // 1, 2, 3
        if (p.x < xmin)
        {
            // 1
            return distance_squared(p.x - xmin, p.y - ymax) > d;
        }
        else if (p.x > xmax)
        {
            // 3
            return distance_squared(p.x - xmax, p.y - ymax) > d;
        }
        else
        {
            // 2
            return distance_squared(0.0, p.y - ymax) > d;
        }
    }
    else if (p.y < ymin)
    {
        // 7, 8, 9
        if (p.x < xmin)
        {
            // 7
            return distance_squared(p.x - xmin, p.y - ymin) > d;
        }
        else if (p.x > xmax)
        {
            // 9
            return distance_squared(p.x - xmax, p.y - ymin) > d;
        }
        else
        {
            // 8
            return distance_squared(0.0, p.y - ymin) > d;
        }
    }
    else
    {
        // 4, 5, 6
        if (p.x < xmin)
        {
            // 4
            return distance_squared(p.x - xmin, 0.0) > d;
        }
        else if (p.x > xmax)
        {
            // 6
            return distance_squared(p.x - xmax, 0.0) > d;
        }
        else
        {
            // 5
            return false;
        }
    }
}

double distance_to_edge(const point p, const edge e)
{
    return dsegment(p.x, p.y, e.p1.x, e.p1.y, e.p2.x, e.p2.y);
}

node::node()
{
    double large_number = std::numeric_limits<double>::max();
    xmin = large_number;
    xmax = -large_number;
    ymin = large_number;
    ymax = -large_number;
}

node::~node()
{
    children_nodes.clear();
    children_edges.clear();
}

double node::get_distance(const double d, const point p) const
{
    if (skip_box(d, p, xmin, xmax, ymin, ymax))
    {
        return d;
    }

    double d_ = d;

    for (int i = 0; i < children_nodes.size(); i++)
    {
        d_ = std::min(d_, children_nodes[i].get_distance(d_, p));
    }

    for (int i = 0; i < children_edges.size(); i++)
    {
        d_ = std::min(d_,
                      dsegment(p.x,
                               p.y,
                               children_edges[i].p1.x,
                               children_edges[i].p1.y,
                               children_edges[i].p2.x,
                               children_edges[i].p2.y));
    }

    return d_;
}

void node::add_child_node(const node child)
{
    children_nodes.push_back(child);

    xmin = std::min(xmin, child.xmin);
    xmax = std::max(xmax, child.xmax);
    ymin = std::min(ymin, child.ymin);
    ymax = std::max(ymax, child.ymax);
}

void node::add_child_edge(const edge child)
{
    children_edges.push_back(child);

    xmin = std::min(xmin, child.p1.x);
    xmax = std::max(xmax, child.p1.x);
    ymin = std::min(ymin, child.p1.y);
    ymax = std::max(ymax, child.p1.y);

    xmin = std::min(xmin, child.p2.x);
    xmax = std::max(xmax, child.p2.x);
    ymin = std::min(ymin, child.p2.y);
    ymax = std::max(ymax, child.p2.y);
}
