```mermaid
flowchart LR
  subgraph path5 [Path]
    5["Path<br>[1037, 1091, 0]"]
    9["Segment<br>[1097, 1124, 0]"]
    10["Segment<br>[1130, 1158, 0]"]
    11["Segment<br>[1164, 1192, 0]"]
    12["Segment<br>[1198, 1205, 0]"]
    20[Solid2d]
  end
  subgraph path6 [Path]
    6["Path<br>[1452, 1539, 0]"]
    13["Segment<br>[1545, 1582, 0]"]
    14["Segment<br>[1588, 1626, 0]"]
    15["Segment<br>[1632, 1672, 0]"]
    16["Segment<br>[1678, 1685, 0]"]
    19[Solid2d]
  end
  subgraph path7 [Path]
    7["Path<br>[1809, 1956, 0]"]
    17["Segment<br>[1809, 1956, 0]"]
    21[Solid2d]
  end
  subgraph path8 [Path]
    8["Path<br>[2246, 2421, 0]"]
    18["Segment<br>[2246, 2421, 0]"]
    22[Solid2d]
  end
  1["Plane<br>[1014, 1031, 0]"]
  2["StartSketchOnFace<br>[1413, 1446, 0]"]
  3["StartSketchOnFace<br>[2199, 2240, 0]"]
  4["StartSketchOnFace<br>[1772, 1803, 0]"]
  23["Sweep Extrusion<br>[1211, 1235, 0]"]
  24["Sweep Extrusion<br>[1691, 1722, 0]"]
  25["Sweep Extrusion<br>[2110, 2138, 0]"]
  26["Sweep Extrusion<br>[2110, 2138, 0]"]
  27["Sweep Extrusion<br>[2110, 2138, 0]"]
  28["Sweep Extrusion<br>[2110, 2138, 0]"]
  29["Sweep Extrusion<br>[2110, 2138, 0]"]
  30["Sweep Extrusion<br>[2110, 2138, 0]"]
  31["Sweep Extrusion<br>[2583, 2611, 0]"]
  32["Sweep Extrusion<br>[2583, 2611, 0]"]
  33[Wall]
  34[Wall]
  35[Wall]
  36[Wall]
  37[Wall]
  38[Wall]
  39[Wall]
  40[Wall]
  41[Wall]
  42[Wall]
  43["Cap Start"]
  44["Cap Start"]
  45["Cap End"]
  46["Cap End"]
  47["Cap End"]
  48["SweepEdge Opposite"]
  49["SweepEdge Opposite"]
  50["SweepEdge Opposite"]
  51["SweepEdge Opposite"]
  52["SweepEdge Opposite"]
  53["SweepEdge Opposite"]
  54["SweepEdge Opposite"]
  55["SweepEdge Opposite"]
  56["SweepEdge Opposite"]
  57["SweepEdge Opposite"]
  58["SweepEdge Adjacent"]
  59["SweepEdge Adjacent"]
  60["SweepEdge Adjacent"]
  61["SweepEdge Adjacent"]
  62["SweepEdge Adjacent"]
  63["SweepEdge Adjacent"]
  64["SweepEdge Adjacent"]
  65["SweepEdge Adjacent"]
  66["SweepEdge Adjacent"]
  67["SweepEdge Adjacent"]
  1 --- 5
  44 x--> 2
  43 x--> 3
  47 x--> 4
  5 --- 9
  5 --- 10
  5 --- 11
  5 --- 12
  5 --- 20
  5 ---- 23
  6 --- 13
  6 --- 14
  6 --- 15
  6 --- 16
  6 --- 19
  6 ---- 24
  44 --- 6
  7 --- 17
  7 --- 21
  7 ---- 30
  47 --- 7
  8 --- 18
  8 --- 22
  8 ---- 31
  43 --- 8
  9 --- 37
  9 x--> 44
  9 --- 51
  9 --- 60
  10 --- 35
  10 x--> 44
  10 --- 52
  10 --- 59
  11 --- 34
  11 x--> 44
  11 --- 49
  11 --- 62
  12 --- 36
  12 x--> 44
  12 --- 50
  12 --- 61
  13 --- 40
  13 x--> 44
  13 --- 53
  13 --- 66
  14 --- 41
  14 x--> 44
  14 --- 56
  14 --- 65
  15 --- 38
  15 x--> 44
  15 --- 55
  15 --- 63
  16 --- 39
  16 x--> 44
  16 --- 54
  16 --- 64
  17 --- 42
  17 x--> 47
  17 --- 57
  17 --- 67
  18 --- 33
  18 x--> 43
  18 --- 48
  18 --- 58
  23 --- 34
  23 --- 35
  23 --- 36
  23 --- 37
  23 --- 44
  23 --- 47
  23 --- 49
  23 --- 50
  23 --- 51
  23 --- 52
  23 --- 59
  23 --- 60
  23 --- 61
  23 --- 62
  24 --- 38
  24 --- 39
  24 --- 40
  24 --- 41
  24 --- 43
  24 --- 53
  24 --- 54
  24 --- 55
  24 --- 56
  24 --- 63
  24 --- 64
  24 --- 65
  24 --- 66
  30 --- 42
  30 --- 45
  30 --- 57
  30 --- 67
  31 --- 33
  31 --- 46
  31 --- 48
  31 --- 58
  33 --- 48
  33 --- 58
  34 --- 49
  59 <--x 34
  34 --- 62
  35 --- 52
  35 --- 59
  60 <--x 35
  36 --- 50
  36 --- 61
  62 <--x 36
  37 --- 51
  37 --- 60
  61 <--x 37
  38 --- 55
  38 --- 63
  65 <--x 38
  39 --- 54
  63 <--x 39
  39 --- 64
  40 --- 53
  64 <--x 40
  40 --- 66
  41 --- 56
  41 --- 65
  66 <--x 41
  42 --- 57
  42 --- 67
  53 <--x 43
  54 <--x 43
  55 <--x 43
  56 <--x 43
  57 <--x 45
  48 <--x 46
  49 <--x 47
  50 <--x 47
  51 <--x 47
  52 <--x 47
```
