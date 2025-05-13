```mermaid
flowchart LR
  subgraph path3 [Path]
    3["Path<br>[46, 71, 0]"]
    5["Segment<br>[79, 97, 0]"]
    6["Segment<br>[105, 123, 0]"]
    7["Segment<br>[131, 150, 0]"]
    8["Segment<br>[158, 166, 0]"]
    14[Solid2d]
  end
  subgraph path4 [Path]
    4["Path<br>[253, 278, 0]"]
    9["Segment<br>[290, 308, 0]"]
    10["Segment<br>[320, 338, 0]"]
    11["Segment<br>[350, 369, 0]"]
    12["Segment<br>[381, 389, 0]"]
    13[Solid2d]
  end
  1["Plane<br>[21, 38, 0]"]
  2["Plane<br>[224, 241, 0]"]
  15["Sweep Extrusion<br>[421, 442, 0]"]
  16["Sweep Extrusion<br>[479, 499, 0]"]
  17[Wall]
  18[Wall]
  19[Wall]
  20[Wall]
  21[Wall]
  22[Wall]
  23[Wall]
  24[Wall]
  25["Cap Start"]
  26["Cap Start"]
  27["Cap End"]
  28["Cap End"]
  29["SweepEdge Opposite"]
  30["SweepEdge Opposite"]
  31["SweepEdge Opposite"]
  32["SweepEdge Opposite"]
  33["SweepEdge Opposite"]
  34["SweepEdge Opposite"]
  35["SweepEdge Opposite"]
  36["SweepEdge Opposite"]
  37["SweepEdge Adjacent"]
  38["SweepEdge Adjacent"]
  39["SweepEdge Adjacent"]
  40["SweepEdge Adjacent"]
  41["SweepEdge Adjacent"]
  42["SweepEdge Adjacent"]
  43["SweepEdge Adjacent"]
  44["SweepEdge Adjacent"]
  1 --- 3
  2 --- 4
  3 --- 5
  3 --- 6
  3 --- 7
  3 --- 8
  3 --- 14
  3 ---- 15
  4 --- 9
  4 --- 10
  4 --- 11
  4 --- 12
  4 --- 13
  4 ---- 16
  5 --- 20
  5 x--> 28
  5 --- 30
  5 --- 37
  6 --- 18
  6 x--> 28
  6 --- 31
  6 --- 38
  7 --- 17
  7 x--> 28
  7 --- 32
  7 --- 40
  8 --- 19
  8 x--> 28
  8 --- 29
  8 --- 39
  9 --- 23
  9 x--> 25
  9 --- 33
  9 --- 41
  10 --- 24
  10 x--> 25
  10 --- 34
  10 --- 44
  11 --- 21
  11 x--> 25
  11 --- 35
  11 --- 42
  12 --- 22
  12 x--> 25
  12 --- 36
  12 --- 43
  15 --- 17
  15 --- 18
  15 --- 19
  15 --- 20
  15 --- 26
  15 --- 28
  15 --- 29
  15 --- 30
  15 --- 31
  15 --- 32
  15 --- 37
  15 --- 38
  15 --- 39
  15 --- 40
  16 --- 21
  16 --- 22
  16 --- 23
  16 --- 24
  16 --- 25
  16 --- 27
  16 --- 33
  16 --- 34
  16 --- 35
  16 --- 36
  16 --- 41
  16 --- 42
  16 --- 43
  16 --- 44
  17 --- 32
  38 <--x 17
  17 --- 40
  18 --- 31
  37 <--x 18
  18 --- 38
  19 --- 29
  19 --- 39
  40 <--x 19
  20 --- 30
  20 --- 37
  39 <--x 20
  21 --- 35
  21 --- 42
  44 <--x 21
  22 --- 36
  42 <--x 22
  22 --- 43
  23 --- 33
  23 --- 41
  43 <--x 23
  24 --- 34
  41 <--x 24
  24 --- 44
  29 <--x 26
  30 <--x 26
  31 <--x 26
  32 <--x 26
  33 <--x 27
  34 <--x 27
  35 <--x 27
  36 <--x 27
```
