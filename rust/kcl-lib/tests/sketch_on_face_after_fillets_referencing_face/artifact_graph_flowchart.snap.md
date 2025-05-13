```mermaid
flowchart LR
  subgraph path3 [Path]
    3["Path<br>[1014, 1039, 0]"]
    5["Segment<br>[1045, 1090, 0]"]
    6["Segment<br>[1096, 1139, 0]"]
    7["Segment<br>[1145, 1172, 0]"]
    8["Segment<br>[1178, 1236, 0]"]
    9["Segment<br>[1242, 1282, 0]"]
    10["Segment<br>[1288, 1296, 0]"]
    17[Solid2d]
  end
  subgraph path4 [Path]
    4["Path<br>[1535, 1566, 0]"]
    11["Segment<br>[1572, 1597, 0]"]
    12["Segment<br>[1603, 1628, 0]"]
    13["Segment<br>[1634, 1659, 0]"]
    14["Segment<br>[1665, 1721, 0]"]
    15["Segment<br>[1727, 1735, 0]"]
    16[Solid2d]
  end
  1["Plane<br>[991, 1008, 0]"]
  2["StartSketchOnFace<br>[1493, 1529, 0]"]
  18["Sweep Extrusion<br>[1302, 1325, 0]"]
  19["Sweep Extrusion<br>[1741, 1761, 0]"]
  20[Wall]
  21[Wall]
  22[Wall]
  23[Wall]
  24[Wall]
  25[Wall]
  26[Wall]
  27[Wall]
  28[Wall]
  29[Wall]
  30["Cap Start"]
  31["Cap End"]
  32["Cap End"]
  33["SweepEdge Opposite"]
  34["SweepEdge Opposite"]
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
  51["SweepEdge Adjacent"]
  52["SweepEdge Adjacent"]
  53["EdgeCut Fillet<br>[1331, 1396, 0]"]
  54["EdgeCut Fillet<br>[1402, 1479, 0]"]
  1 --- 3
  29 x--> 2
  3 --- 5
  3 --- 6
  3 --- 7
  3 --- 8
  3 --- 9
  3 --- 10
  3 --- 17
  3 ---- 18
  4 --- 11
  4 --- 12
  4 --- 13
  4 --- 14
  4 --- 15
  4 --- 16
  4 ---- 19
  29 --- 4
  5 --- 28
  5 x--> 30
  5 --- 37
  5 --- 48
  6 --- 29
  6 x--> 30
  6 --- 41
  6 --- 50
  7 --- 26
  7 x--> 30
  7 --- 38
  7 --- 47
  8 --- 27
  8 x--> 30
  8 --- 40
  8 --- 51
  9 --- 25
  9 x--> 30
  9 --- 39
  9 --- 49
  10 --- 24
  10 x--> 30
  10 --- 42
  10 --- 52
  11 --- 23
  11 x--> 29
  11 --- 36
  11 --- 44
  12 --- 20
  12 x--> 29
  12 --- 35
  12 --- 46
  13 --- 22
  13 x--> 29
  13 --- 34
  13 --- 45
  14 --- 21
  14 x--> 29
  14 --- 33
  14 --- 43
  18 --- 24
  18 --- 25
  18 --- 26
  18 --- 27
  18 --- 28
  18 --- 29
  18 --- 30
  18 --- 31
  18 --- 37
  18 --- 38
  18 --- 39
  18 --- 40
  18 --- 41
  18 --- 42
  18 --- 47
  18 --- 48
  18 --- 49
  18 --- 50
  18 --- 51
  18 --- 52
  19 --- 20
  19 --- 21
  19 --- 22
  19 --- 23
  19 --- 32
  19 --- 33
  19 --- 34
  19 --- 35
  19 --- 36
  19 --- 43
  19 --- 44
  19 --- 45
  19 --- 46
  20 --- 35
  44 <--x 20
  20 --- 46
  21 --- 33
  21 --- 43
  45 <--x 21
  22 --- 34
  22 --- 45
  46 <--x 22
  23 --- 36
  43 <--x 23
  23 --- 44
  24 --- 42
  49 <--x 24
  24 --- 52
  25 --- 39
  25 --- 49
  51 <--x 25
  26 --- 38
  26 --- 47
  50 <--x 26
  27 --- 40
  47 <--x 27
  27 --- 51
  28 --- 37
  28 --- 48
  52 <--x 28
  29 --- 41
  48 <--x 29
  29 --- 50
  37 <--x 31
  38 <--x 31
  39 <--x 31
  40 <--x 31
  41 <--x 31
  42 <--x 31
  33 <--x 32
  34 <--x 32
  35 <--x 32
  36 <--x 32
  48 <--x 54
  51 <--x 53
```
