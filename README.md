# Maze Generator and Solver

## Maze Generator

A maze can be generated with an extremely high degree of customizability. Weights for *24 types of wall/junctions* can be chosen. The maze is generated by removing edges from a grid (whose cells are intially completely disconnected) until all cells are fully connected. If we were to think of the cells as nodes of a graph, the generated maze forms a minimum-spanning tree.

The algorithm randomly selects an edge to remove based on the weights. The type of wall formed by an edge and its surrounding edges is defined as follow:

In the diagram below, `~` represents our edge of interest. The horizontal and vertical lines represent the surrounding edges. The cells are the space between these edges. In the diagram below, there are 6 cells and 7 edges.
```
     0   3
     |   |
 1 -   ~   - 4
     |   |
     2   5
```
A number is assigned to each of the neighboring edges. Each wall type is given a name, e.g. `type_101x001`, `type_001x100`, etc. The 6 digits in the back of the name indicate the presence of each neighboring edge. The numbers `0` to `5` in the diagram above indicate the position of such digits: `type_[edge 0][edge 1][edge 2]x[edge 3][edge 4][edge 5]`.

For example, the wall type `type_101x001` indicates that it has edges at positions `0`, `1`, and `5`.
```
|
  ~
|   |
```

THe wall type `type_001x100`, on the other hand, has edges at positions `2` and `3`.
```
    |
  ~
|
```

Any edge of interest (vertical or horizontal) whose neighboring edges are merely mirror or rotated images of these types are considered to be of the same type. For example, `type_100x101` would be considered to be the wall type as `type_101x001`.
```
|                         |   |
  ~    <== same type ==>    ~
|   |                         |
```