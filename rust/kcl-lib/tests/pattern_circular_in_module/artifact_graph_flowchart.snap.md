```mermaid
flowchart LR
  subgraph path3 [Path]
    3["Path<br>[63, 90, 1]"]
    5["Segment<br>[98, 116, 1]"]
    7["Segment<br>[124, 143, 1]"]
    9["Segment<br>[151, 170, 1]"]
    12["Segment<br>[178, 185, 1]"]
    13[Solid2d]
  end
  subgraph path4 [Path]
    4["Path<br>[63, 90, 1]"]
    6["Segment<br>[98, 116, 1]"]
    8["Segment<br>[124, 143, 1]"]
    10["Segment<br>[151, 170, 1]"]
    11["Segment<br>[178, 185, 1]"]
    14[Solid2d]
  end
  1["Plane<br>[38, 55, 1]"]
  2["Plane<br>[38, 55, 1]"]
  15["Sweep Extrusion<br>[342, 376, 1]"]
  16["Sweep Extrusion<br>[342, 376, 1]"]
  17["Sweep Extrusion<br>[342, 376, 1]"]
  18["Sweep Extrusion<br>[342, 376, 1]"]
  19["Sweep Extrusion<br>[342, 376, 1]"]
  20["Sweep Extrusion<br>[342, 376, 1]"]
  21["Sweep Extrusion<br>[342, 376, 1]"]
  22["Sweep Extrusion<br>[342, 376, 1]"]
  23[Wall]
  24[Wall]
  25[Wall]
  26[Wall]
  27[Wall]
  28[Wall]
  29[Wall]
  30[Wall]
  31["Cap Start"]
  32["Cap Start"]
  33["Cap End"]
  34["Cap End"]
  35["SweepEdge Opposite"]
  36["SweepEdge Opposite"]
  37["SweepEdge Opposite"]
  38["SweepEdge Opposite"]
  39["SweepEdge Opposite"]
  40["SweepEdge Opposite"]
  41["SweepEdge Opposite"]
  42["SweepEdge Opposite"]
  43["SweepEdge Adjacent"]
  44["SweepEdge Adjacent"]
  45["SweepEdge Adjacent"]
  46["SweepEdge Adjacent"]
  47["SweepEdge Adjacent"]
  48["SweepEdge Adjacent"]
  49["SweepEdge Adjacent"]
  50["SweepEdge Adjacent"]
  1 --- 3
  2 --- 4
  3 --- 5
  3 --- 7
  3 --- 9
  3 --- 12
  3 --- 13
  3 ---- 22
  4 --- 6
  4 --- 8
  4 --- 10
  4 --- 11
  4 --- 14
  4 ---- 16
  5 --- 29
  5 x--> 32
  5 --- 41
  5 --- 47
  6 --- 26
  6 x--> 31
  6 --- 38
  6 --- 43
  7 --- 28
  7 x--> 32
  7 --- 39
  7 --- 49
  8 --- 24
  8 x--> 31
  8 --- 35
  8 --- 46
  9 --- 27
  9 x--> 32
  9 --- 40
  9 --- 48
  10 --- 23
  10 x--> 31
  10 --- 36
  10 --- 45
  11 --- 25
  11 x--> 31
  11 --- 37
  11 --- 44
  12 --- 30
  12 x--> 32
  12 --- 42
  12 --- 50
  16 --- 23
  16 --- 24
  16 --- 25
  16 --- 26
  16 --- 31
  16 --- 33
  16 --- 35
  16 --- 36
  16 --- 37
  16 --- 38
  16 --- 43
  16 --- 44
  16 --- 45
  16 --- 46
  22 --- 27
  22 --- 28
  22 --- 29
  22 --- 30
  22 --- 32
  22 --- 34
  22 --- 39
  22 --- 40
  22 --- 41
  22 --- 42
  22 --- 47
  22 --- 48
  22 --- 49
  22 --- 50
  23 --- 36
  23 --- 45
  46 <--x 23
  24 --- 35
  43 <--x 24
  24 --- 46
  25 --- 37
  25 --- 44
  45 <--x 25
  26 --- 38
  26 --- 43
  44 <--x 26
  27 --- 40
  27 --- 48
  49 <--x 27
  28 --- 39
  47 <--x 28
  28 --- 49
  29 --- 41
  29 --- 47
  50 <--x 29
  30 --- 42
  48 <--x 30
  30 --- 50
  35 <--x 33
  36 <--x 33
  37 <--x 33
  38 <--x 33
  39 <--x 34
  40 <--x 34
  41 <--x 34
  42 <--x 34
```
