```mermaid
flowchart LR
  subgraph path2 [Path]
    2["Path<br>[133, 158, 0]"]
  end
  subgraph path3 [Path]
    3["Path<br>[164, 270, 0]"]
    4["Segment<br>[164, 270, 0]"]
    5["Segment<br>[164, 270, 0]"]
    6["Segment<br>[164, 270, 0]"]
    7["Segment<br>[164, 270, 0]"]
    8["Segment<br>[164, 270, 0]"]
    9[Solid2d]
  end
  1["Plane<br>[110, 127, 0]"]
  10["Sweep Extrusion<br>[276, 295, 0]"]
  11[Wall]
  12[Wall]
  13[Wall]
  14[Wall]
  15["Cap Start"]
  16["Cap End"]
  17["SweepEdge Opposite"]
  18["SweepEdge Opposite"]
  19["SweepEdge Opposite"]
  20["SweepEdge Opposite"]
  21["SweepEdge Adjacent"]
  22["SweepEdge Adjacent"]
  23["SweepEdge Adjacent"]
  24["SweepEdge Adjacent"]
  1 --- 2
  1 --- 3
  3 --- 4
  3 --- 5
  3 --- 6
  3 --- 7
  3 --- 8
  3 --- 9
  3 ---- 10
  4 --- 11
  4 x--> 15
  4 --- 20
  4 --- 24
  5 --- 12
  5 x--> 15
  5 --- 17
  5 --- 21
  6 --- 13
  6 x--> 15
  6 --- 19
  6 --- 22
  8 --- 14
  8 x--> 15
  8 --- 18
  8 --- 23
  10 --- 11
  10 --- 12
  10 --- 13
  10 --- 14
  10 --- 15
  10 --- 16
  10 --- 17
  10 --- 18
  10 --- 19
  10 --- 20
  10 --- 21
  10 --- 22
  10 --- 23
  10 --- 24
  11 --- 20
  21 <--x 11
  11 --- 24
  12 --- 17
  12 --- 21
  23 <--x 12
  13 --- 19
  13 --- 22
  24 <--x 13
  14 --- 18
  22 <--x 14
  14 --- 23
  17 <--x 16
  18 <--x 16
  19 <--x 16
  20 <--x 16
```
