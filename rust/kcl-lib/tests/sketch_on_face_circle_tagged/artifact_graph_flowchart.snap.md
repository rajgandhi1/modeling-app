```mermaid
flowchart LR
  subgraph path3 [Path]
    3["Path<br>[54, 76, 0]"]
    5["Segment<br>[84, 106, 0]"]
    6["Segment<br>[114, 136, 0]"]
    7["Segment<br>[144, 167, 0]"]
    8["Segment<br>[229, 237, 0]"]
    11[Solid2d]
  end
  subgraph path4 [Path]
    4["Path<br>[317, 369, 0]"]
    9["Segment<br>[317, 369, 0]"]
    10[Solid2d]
  end
  1["Plane<br>[29, 46, 0]"]
  2["StartSketchOnFace<br>[275, 311, 0]"]
  12["Sweep Extrusion<br>[243, 263, 0]"]
  13["Sweep Extrusion<br>[375, 394, 0]"]
  14[Wall]
  15[Wall]
  16[Wall]
  17[Wall]
  18[Wall]
  19["Cap Start"]
  20["Cap Start"]
  21["Cap End"]
  22["Cap End"]
  23["SweepEdge Opposite"]
  24["SweepEdge Opposite"]
  25["SweepEdge Opposite"]
  26["SweepEdge Opposite"]
  27["SweepEdge Opposite"]
  28["SweepEdge Adjacent"]
  29["SweepEdge Adjacent"]
  30["SweepEdge Adjacent"]
  31["SweepEdge Adjacent"]
  32["SweepEdge Adjacent"]
  1 --- 3
  22 x--> 2
  3 --- 5
  3 --- 6
  3 --- 7
  3 --- 8
  3 --- 11
  3 ---- 12
  4 --- 9
  4 --- 10
  4 ---- 13
  22 --- 4
  5 --- 17
  5 x--> 20
  5 --- 26
  5 --- 31
  6 --- 15
  6 x--> 20
  6 --- 25
  6 --- 30
  7 --- 14
  7 x--> 20
  7 --- 24
  7 --- 29
  8 --- 16
  8 x--> 20
  8 --- 23
  8 --- 28
  9 --- 18
  9 x--> 19
  9 --- 27
  9 --- 32
  12 --- 14
  12 --- 15
  12 --- 16
  12 --- 17
  12 --- 20
  12 --- 22
  12 --- 23
  12 --- 24
  12 --- 25
  12 --- 26
  12 --- 28
  12 --- 29
  12 --- 30
  12 --- 31
  13 --- 18
  13 --- 19
  13 --- 21
  13 --- 27
  13 --- 32
  14 --- 24
  14 --- 29
  30 <--x 14
  15 --- 25
  15 --- 30
  31 <--x 15
  16 --- 23
  16 --- 28
  29 <--x 16
  17 --- 26
  28 <--x 17
  17 --- 31
  18 --- 27
  18 --- 32
  27 <--x 21
  23 <--x 22
  24 <--x 22
  25 <--x 22
  26 <--x 22
```
