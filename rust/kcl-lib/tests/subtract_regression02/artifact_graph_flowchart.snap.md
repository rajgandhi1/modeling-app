```mermaid
flowchart LR
  subgraph path4 [Path]
    4["Path<br>[88, 131, 0]"]
    7["Segment<br>[137, 157, 0]"]
    8["Segment<br>[163, 182, 0]"]
    9["Segment<br>[188, 265, 0]"]
    10["Segment<br>[271, 293, 0]"]
    11["Segment<br>[299, 380, 0]"]
    12["Segment<br>[386, 407, 0]"]
    13["Segment<br>[413, 490, 0]"]
    14["Segment<br>[496, 503, 0]"]
    18[Solid2d]
  end
  subgraph path5 [Path]
    5["Path<br>[646, 704, 0]"]
    15["Segment<br>[646, 704, 0]"]
    17[Solid2d]
  end
  subgraph path6 [Path]
    6["Path<br>[901, 959, 0]"]
    16["Segment<br>[901, 959, 0]"]
    19[Solid2d]
  end
  1["Plane<br>[47, 64, 0]"]
  2["Plane<br>[605, 622, 0]"]
  3["Plane<br>[859, 877, 0]"]
  20["Sweep Extrusion<br>[518, 591, 0]"]
  21["Sweep Extrusion<br>[722, 791, 0]"]
  22["Sweep Extrusion<br>[977, 1046, 0]"]
  23["CompositeSolid Subtract<br>[1057, 1096, 0]"]
  24["CompositeSolid Subtract<br>[802, 845, 0]"]
  25[Wall]
  26[Wall]
  27[Wall]
  28[Wall]
  29[Wall]
  30[Wall]
  31[Wall]
  32[Wall]
  33[Wall]
  34["Cap Start"]
  35["Cap Start"]
  36["Cap Start"]
  37["Cap End"]
  38["Cap End"]
  39["Cap End"]
  40["SweepEdge Opposite"]
  41["SweepEdge Opposite"]
  42["SweepEdge Opposite"]
  43["SweepEdge Opposite"]
  44["SweepEdge Opposite"]
  45["SweepEdge Opposite"]
  46["SweepEdge Opposite"]
  47["SweepEdge Opposite"]
  48["SweepEdge Opposite"]
  49["SweepEdge Adjacent"]
  50["SweepEdge Adjacent"]
  51["SweepEdge Adjacent"]
  52["SweepEdge Adjacent"]
  53["SweepEdge Adjacent"]
  54["SweepEdge Adjacent"]
  55["SweepEdge Adjacent"]
  56["SweepEdge Adjacent"]
  57["SweepEdge Adjacent"]
  1 --- 4
  2 --- 5
  3 --- 6
  4 --- 7
  4 --- 8
  4 --- 9
  4 --- 10
  4 --- 11
  4 --- 12
  4 --- 13
  4 --- 14
  4 --- 18
  4 ---- 20
  4 --- 24
  5 --- 15
  5 --- 17
  5 ---- 21
  5 --- 24
  6 --- 16
  6 --- 19
  6 ---- 22
  6 --- 23
  7 --- 33
  7 x--> 36
  7 --- 42
  7 --- 54
  8 --- 30
  8 x--> 36
  8 --- 48
  8 --- 56
  9 --- 29
  9 x--> 36
  9 --- 44
  9 --- 55
  10 --- 31
  10 x--> 36
  10 --- 46
  10 --- 53
  11 --- 28
  11 x--> 36
  11 --- 47
  11 --- 57
  12 --- 27
  12 x--> 36
  12 --- 45
  12 --- 52
  13 --- 32
  13 x--> 36
  13 --- 43
  13 --- 51
  15 --- 26
  15 x--> 34
  15 --- 41
  15 --- 50
  16 --- 25
  16 x--> 35
  16 --- 40
  16 --- 49
  20 --- 27
  20 --- 28
  20 --- 29
  20 --- 30
  20 --- 31
  20 --- 32
  20 --- 33
  20 --- 36
  20 --- 39
  20 --- 42
  20 --- 43
  20 --- 44
  20 --- 45
  20 --- 46
  20 --- 47
  20 --- 48
  20 --- 51
  20 --- 52
  20 --- 53
  20 --- 54
  20 --- 55
  20 --- 56
  20 --- 57
  21 --- 26
  21 --- 34
  21 --- 37
  21 --- 41
  21 --- 50
  22 --- 25
  22 --- 35
  22 --- 38
  22 --- 40
  22 --- 49
  24 --- 23
  25 --- 40
  25 --- 49
  26 --- 41
  26 --- 50
  27 --- 45
  27 --- 52
  57 <--x 27
  28 --- 47
  53 <--x 28
  28 --- 57
  29 --- 44
  29 --- 55
  56 <--x 29
  30 --- 48
  54 <--x 30
  30 --- 56
  31 --- 46
  31 --- 53
  55 <--x 31
  32 --- 43
  32 --- 51
  52 <--x 32
  33 --- 42
  51 <--x 33
  33 --- 54
  41 <--x 37
  40 <--x 38
  42 <--x 39
  43 <--x 39
  44 <--x 39
  45 <--x 39
  46 <--x 39
  47 <--x 39
  48 <--x 39
```
