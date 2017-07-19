#include <limits>

#include "edge.h"
#include "node.h"
#include "distance.h"
#include "intersection.h"

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
bool skip_box_distance(const double d,
                       const point p,
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

    if (difx == 0.0 and dify == 0.0)
    {
        return false;
    }
    else
    {
        return distance_squared(difx, dify) > d;
    }
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
}

node::~node()
{
    children_nodes.clear();
    children_edges.clear();
}

double node::get_distance_edge(const double d, const point p) const
{
    if (skip_box_distance(d, p, xmin, xmax, ymin, ymax))
        return d;

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
    if (skip_box_distance(d, p, xmin, xmax, ymin, ymax))
        return d;

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
         // no need to check the second point
         // d_ = std::min(d_, distance_squared(children_edges[i].p2.x - p.x, children_edges[i].p2.y - p.y));
        }
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
