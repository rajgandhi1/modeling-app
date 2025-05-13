```mermaid
flowchart LR
  subgraph path5 [Path]
    5["Path<br>[791, 835, 0]"]
    9["Segment<br>[841, 885, 0]"]
    10["Segment<br>[891, 934, 0]"]
    11["Segment<br>[940, 984, 0]"]
    12["Segment<br>[990, 997, 0]"]
    18[Solid2d]
  end
  subgraph path6 [Path]
    6["Path<br>[1084, 1231, 0]"]
    13["Segment<br>[1084, 1231, 0]"]
    17[Solid2d]
  end
  subgraph path7 [Path]
    7["Path<br>[1478, 1627, 0]"]
    14["Segment<br>[1478, 1627, 0]"]
    19[Solid2d]
  end
  subgraph path8 [Path]
    8["Path<br>[1879, 1927, 0]"]
    15["Segment<br>[1879, 1927, 0]"]
    16[Solid2d]
  end
  1["Plane<br>[768, 785, 0]"]
  2["StartSketchOnFace<br>[1047, 1078, 0]"]
  3["StartSketchOnFace<br>[1842, 1873, 0]"]
  4["StartSketchOnFace<br>[1439, 1472, 0]"]
  20["Sweep Extrusion<br>[1003, 1027, 0]"]
  21["Sweep Extrusion<br>[1396, 1425, 0]"]
  22["Sweep Extrusion<br>[1396, 1425, 0]"]
  23["Sweep Extrusion<br>[1396, 1425, 0]"]
  24["Sweep Extrusion<br>[1396, 1425, 0]"]
  25["Sweep Extrusion<br>[1792, 1827, 0]"]
  26["Sweep Extrusion<br>[1792, 1827, 0]"]
  27["Sweep Extrusion<br>[1792, 1827, 0]"]
  28["Sweep Extrusion<br>[1792, 1827, 0]"]
  29["Sweep Extrusion<br>[1933, 1958, 0]"]
  30[Wall]
  31[Wall]
  32[Wall]
  33[Wall]
  34[Wall]
  35[Wall]
  36[Wall]
  37["Cap Start"]
  38["Cap Start"]
  39["Cap End"]
  40["SweepEdge Opposite"]
  41["SweepEdge Opposite"]
  42["SweepEdge Opposite"]
  43["SweepEdge Opposite"]
  44["SweepEdge Opposite"]
  45["SweepEdge Opposite"]
  46["SweepEdge Opposite"]
  47["SweepEdge Adjacent"]
  48["SweepEdge Adjacent"]
  49["SweepEdge Adjacent"]
  50["SweepEdge Adjacent"]
  51["SweepEdge Adjacent"]
  52["SweepEdge Adjacent"]
  53["SweepEdge Adjacent"]
  1 --- 5
  39 x--> 2
  39 x--> 3
  38 x--> 4
  5 --- 9
  5 --- 10
  5 --- 11
  5 --- 12
  5 --- 18
  5 ---- 20
  6 --- 13
  6 --- 17
  6 ---- 23
  39 --- 6
  7 --- 14
  7 --- 19
  7 ---- 28
  38 --- 7
  8 --- 15
  8 --- 16
  8 ---- 29
  39 --- 8
  9 --- 34
  9 x--> 38
  9 --- 41
  9 --- 48
  10 --- 32
  10 x--> 38
  10 --- 42
  10 --- 49
  11 --- 31
  11 x--> 38
  11 --- 43
  11 --- 50
  12 --- 33
  12 x--> 38
  12 --- 44
  12 --- 51
  13 --- 35
  13 x--> 39
  13 --- 45
  13 --- 52
  14 --- 36
  14 x--> 38
  14 --- 46
  14 --- 53
  15 --- 30
  15 x--> 39
  15 --- 40
  15 --- 47
  20 --- 31
  20 --- 32
  20 --- 33
  20 --- 34
  20 --- 38
  20 --- 39
  20 --- 41
  20 --- 42
  20 --- 43
  20 --- 44
  20 --- 48
  20 --- 49
  20 --- 50
  20 --- 51
  23 --- 35
  23 --- 37
  23 --- 45
  23 --- 52
  28 --- 36
  28 --- 46
  28 --- 53
  29 --- 30
  29 --- 40
  29 --- 47
  30 --- 40
  30 --- 47
  31 --- 43
  49 <--x 31
  31 --- 50
  32 --- 42
  48 <--x 32
  32 --- 49
  33 --- 44
  50 <--x 33
  33 --- 51
  34 --- 41
  34 --- 48
  51 <--x 34
  35 --- 45
  35 --- 52
  36 --- 46
  36 --- 53
  45 <--x 37
  40 <--x 38
  41 <--x 39
  42 <--x 39
  43 <--x 39
  44 <--x 39
```
