#include <limits>

#include "edge.h"
#include "node.h"
#include "distance.h"

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
    if (skip_box_distance(d, p, xmin, xmax, ymin, ymax))
    {
        return d;
    }

    double d_ = d;

    if (children_nodes.size() > 0)
    {
        for (int i = 0; i < children_nodes.size(); i++)
        {
            d_ = std::min(d_, children_nodes[i].get_distance(d_, p));
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
