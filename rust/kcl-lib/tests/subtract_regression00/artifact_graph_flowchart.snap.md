```mermaid
flowchart LR
  subgraph path4 [Path]
    4["Path<br>[88, 128, 0]"]
    6["Segment<br>[134, 152, 0]"]
    7["Segment<br>[158, 176, 0]"]
    8["Segment<br>[182, 201, 0]"]
    9["Segment<br>[207, 226, 0]"]
    10["Segment<br>[232, 239, 0]"]
    12[Solid2d]
  end
  subgraph path5 [Path]
    5["Path<br>[401, 458, 0]"]
    11["Segment<br>[401, 458, 0]"]
    13[Solid2d]
  end
  1["Plane<br>[47, 64, 0]"]
  2["Plane<br>[348, 376, 0]"]
  3["StartSketchOnPlane<br>[334, 377, 0]"]
  14["Sweep Extrusion<br>[254, 320, 0]"]
  15["Sweep Extrusion<br>[476, 518, 0]"]
  16["CompositeSolid Subtract<br>[529, 572, 0]"]
  17[Wall]
  18[Wall]
  19[Wall]
  20[Wall]
  21[Wall]
  22["Cap Start"]
  23["Cap Start"]
  24["Cap End"]
  25["Cap End"]
  26["SweepEdge Opposite"]
  27["SweepEdge Opposite"]
  28["SweepEdge Opposite"]
  29["SweepEdge Opposite"]
  30["SweepEdge Opposite"]
  31["SweepEdge Adjacent"]
  32["SweepEdge Adjacent"]
  33["SweepEdge Adjacent"]
  34["SweepEdge Adjacent"]
  35["SweepEdge Adjacent"]
  1 --- 4
  2 <--x 3
  2 --- 5
  4 --- 6
  4 --- 7
  4 --- 8
  4 --- 9
  4 --- 10
  4 --- 12
  4 ---- 14
  4 --- 16
  5 --- 11
  5 --- 13
  5 ---- 15
  5 --- 16
  6 --- 20
  6 x--> 23
  6 --- 29
  6 --- 31
  7 --- 18
  7 x--> 23
  7 --- 26
  7 --- 34
  8 --- 17
  8 x--> 23
  8 --- 27
  8 --- 33
  9 --- 19
  9 x--> 23
  9 --- 28
  9 --- 32
  11 --- 21
  11 x--> 24
  11 --- 30
  11 --- 35
  14 --- 17
  14 --- 18
  14 --- 19
  14 --- 20
  14 --- 23
  14 --- 25
  14 --- 26
  14 --- 27
  14 --- 28
  14 --- 29
  14 --- 31
  14 --- 32
  14 --- 33
  14 --- 34
  15 --- 21
  15 --- 22
  15 --- 24
  15 --- 30
  15 --- 35
  17 --- 27
  17 --- 33
  34 <--x 17
  18 --- 26
  31 <--x 18
  18 --- 34
  19 --- 28
  19 --- 32
  33 <--x 19
  20 --- 29
  20 --- 31
  32 <--x 20
  21 --- 30
  21 --- 35
  30 <--x 22
  26 <--x 25
  27 <--x 25
  28 <--x 25
  29 <--x 25
```
