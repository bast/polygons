#pragma once

#include <vector>
#include <tuple>
#include "edge.h"

class node
{
  public:
    node();
    ~node();

    void add_child_node(const node child);
    void add_child_edge(const edge child);

    double get_distance_edge(const double d, const point p) const;
    std::tuple<int, double> get_distance_vertex(const int index, const double d, const point p) const;
    double get_distance_vertex_weighted(const double scale_factor, const double d, const point p) const;
    int num_intersections(const int n, const point p) const;

  private:
    // node(const node &rhs);            // not implemented
    // node &operator=(const node &rhs); // not implemented

    double xmin;
    double xmax;
    double ymin;
    double ymax;

    double weight;

    std::vector<node> children_nodes;
    std::vector<edge> children_edges;
};
