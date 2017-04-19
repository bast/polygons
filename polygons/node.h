#ifndef NODE_H_INCLUDED
#define NODE_H_INCLUDED

#include <vector>
#include "edge.h"

class node
{
  public:
    node();
    ~node();

    void add_child_node(const node child);
    void add_child_edge(const edge child);

    double get_distance(const double d, const point p) const;
    int num_intersections(const int n, const point p) const;

  private:
    //  node(const node &rhs);            // not implemented
    node &operator=(const node &rhs); // not implemented

    double xmin;
    double xmax;
    double ymin;
    double ymax;
    std::vector<node> children_nodes;
    std::vector<edge> children_edges;
};

#endif /* NODE_H_INCLUDED */
