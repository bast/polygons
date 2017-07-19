#include <limits>
#include <stdio.h>
#include <math.h>

#include "edge.h"
#include "node.h"
#include "distance.h"
#include "intersection.h"

// get best case distance to box
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
double box_distance(const point p,
                    const double xmin,
                    const double xmax,
                    const double ymin,
                    const double ymax)
{
    double difx;

    if (p.x < xmin)
    {
        difx = p.x - xmin;
    }
    else if (p.x > xmax)
    {
        difx = p.x - xmax;
    }
    else
    {
        difx = 0.0;
    }

    double dify;

    if (p.y < ymin)
    {
        dify = p.y - ymin;
    }
    else if (p.y > ymax)
    {
        dify = p.y - ymax;
    }
    else
    {
        dify = 0.0;
    }

    return distance_squared(difx, dify);
}

bool skip_box_intersection(const point p,
                           const double xmax,
                           const double ymin,
                           const double ymax)
{
    if (p.x > xmax)
        return true;
    if (p.y > ymax)
        return true;
    if (p.y < ymin)
        return true;
    return false;
}

node::node()
{
    double large_number = std::numeric_limits<double>::max();
    xmin = large_number;
    xmax = -large_number;
    ymin = large_number;
    ymax = -large_number;
    weight = large_number;
}

node::~node()
{
    children_nodes.clear();
    children_edges.clear();
}

double node::get_distance_edge(const double d, const point p) const
{
    if (box_distance(p, xmin, xmax, ymin, ymax) > d) return d;

    double d_ = d;

    if (children_nodes.size() > 0)
    {
        for (int i = 0; i < children_nodes.size(); i++)
        {
            d_ = std::min(d_, children_nodes[i].get_distance_edge(d_, p));
        }
        return d_;
    }

    if (children_edges.size() > 0)
    {
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
}

double node::get_distance_vertex(const double d, const point p) const
{
    if (box_distance(p, xmin, xmax, ymin, ymax) > d) return d;

    double d_ = d;

    if (children_nodes.size() > 0)
    {
        for (int i = 0; i < children_nodes.size(); i++)
        {
            d_ = std::min(d_, children_nodes[i].get_distance_vertex(d_, p));
        }
        return d_;
    }

    if (children_edges.size() > 0)
    {
        for (int i = 0; i < children_edges.size(); i++)
        {
            d_ = std::min(d_, distance_squared(children_edges[i].p1.x - p.x, children_edges[i].p1.y - p.y));
        }
        d_ = std::min(d_, distance_squared(children_edges[children_edges.size()-1].p2.x - p.x, children_edges[children_edges.size()-1].p2.y - p.y));
        return d_;
    }
}

// FIXME this is hardcoded and not clear at all what it does - used in another project
//       need to generalize and explain
double linear_function(const double nearest_distance_at_coastline_point,
                       const double distace_to_coastline_point)
{
    int num_points_across_bay = 5;
    double resolution_at_coastline_point =
        nearest_distance_at_coastline_point / (num_points_across_bay + 1);
    double slope = 0.995792;

    return resolution_at_coastline_point + slope * sqrt(distace_to_coastline_point);
}

double node::get_distance_vertex_weighted(const double d, const point p) const
{
    double r_ = linear_function(weight, box_distance(p, xmin, xmax, ymin, ymax));
    if (r_ > d) return d;

    double d_ = d;

    if (children_nodes.size() > 0)
    {
        for (int i = 0; i < children_nodes.size(); i++)
        {
            d_ = std::min(d_, children_nodes[i].get_distance_vertex_weighted(d_, p));
        }
        return d_;
    }

    if (children_edges.size() > 0)
    {
        for (int i = 0; i < children_edges.size(); i++)
        {
            d_ = std::min(d_, linear_function(children_edges[i].p1.weight, distance_squared(children_edges[i].p1.x - p.x, children_edges[i].p1.y - p.y)));
        }
        d_ = std::min(d_, linear_function(children_edges[children_edges.size()-1].p2.weight, distance_squared(children_edges[children_edges.size()-1].p2.x - p.x, children_edges[children_edges.size()-1].p2.y - p.y)));
        return d_;
    }
}

int node::num_intersections(const int n, const point p) const
{
    if (skip_box_intersection(p, xmax, ymin, ymax))
        return n;

    int n_ = n;

    if (children_nodes.size() > 0)
    {
        for (int i = 0; i < children_nodes.size(); i++)
        {
            n_ = children_nodes[i].num_intersections(n_, p);
        }
        return n_;
    }

    if (children_edges.size() > 0)
    {
        for (int i = 0; i < children_edges.size(); i++)
        {
            if (crosses(p.x, p.y, children_edges[i]))
                n_++;
        }
        return n_;
    }
}

void node::add_child_node(const node child)
{
    children_nodes.push_back(child);

    xmin = std::min(xmin, child.xmin);
    xmax = std::max(xmax, child.xmax);
    ymin = std::min(ymin, child.ymin);
    ymax = std::max(ymax, child.ymax);

    weight = std::min(weight, child.weight);
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

    weight = std::min(weight, child.p1.weight);
    weight = std::min(weight, child.p2.weight);
}
