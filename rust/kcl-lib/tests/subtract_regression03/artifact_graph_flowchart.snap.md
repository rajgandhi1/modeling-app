```mermaid
flowchart LR
  subgraph path5 [Path]
    5["Path<br>[88, 140, 0]"]
    9["Segment<br>[146, 179, 0]"]
    10["Segment<br>[185, 275, 0]"]
    11["Segment<br>[281, 313, 0]"]
    12["Segment<br>[319, 400, 0]"]
    13["Segment<br>[406, 439, 0]"]
    14["Segment<br>[445, 534, 0]"]
    15["Segment<br>[540, 574, 0]"]
  end
  subgraph path6 [Path]
    6["Path<br>[813, 872, 0]"]
    16["Segment<br>[813, 872, 0]"]
    27[Solid2d]
  end
  subgraph path7 [Path]
    7["Path<br>[1156, 1196, 0]"]
    17["Segment<br>[1202, 1230, 0]"]
    18["Segment<br>[1236, 1261, 0]"]
    19["Segment<br>[1267, 1288, 0]"]
    20["Segment<br>[1294, 1301, 0]"]
    25[Solid2d]
  end
  subgraph path8 [Path]
    8["Path<br>[1621, 1661, 0]"]
    21["Segment<br>[1667, 1687, 0]"]
    22["Segment<br>[1693, 1718, 0]"]
    23["Segment<br>[1724, 1753, 0]"]
    24["Segment<br>[1759, 1766, 0]"]
    26[Solid2d]
  end
  1["Plane<br>[47, 64, 0]"]
  2["Plane<br>[766, 789, 0]"]
  3["Plane<br>[1109, 1132, 0]"]
  4["Plane<br>[1574, 1597, 0]"]
  28["Sweep Sweep<br>[892, 952, 0]"]
  29["Sweep Extrusion<br>[1319, 1364, 0]"]
  30["Sweep Extrusion<br>[1784, 1829, 0]"]
  31["CompositeSolid Subtract<br>[1840, 1879, 0]"]
  32["CompositeSolid Subtract<br>[1375, 1423, 0]"]
  33[Wall]
  34[Wall]
  35[Wall]
  36[Wall]
  37[Wall]
  38[Wall]
  39[Wall]
  40["Cap Start"]
  41["Cap Start"]
  42["Cap Start"]
  43["Cap End"]
  44["Cap End"]
  45["Cap End"]
  46["SweepEdge Opposite"]
  47["SweepEdge Opposite"]
  48["SweepEdge Opposite"]
  49["SweepEdge Opposite"]
  50["SweepEdge Opposite"]
  51["SweepEdge Opposite"]
  52["SweepEdge Opposite"]
  53["SweepEdge Adjacent"]
  54["SweepEdge Adjacent"]
  55["SweepEdge Adjacent"]
  56["SweepEdge Adjacent"]
  57["SweepEdge Adjacent"]
  58["SweepEdge Adjacent"]
  59["SweepEdge Adjacent"]
  1 --- 5
  2 --- 6
  3 --- 7
  4 --- 8
  5 --- 9
  5 --- 10
  5 --- 11
  5 --- 12
  5 --- 13
  5 --- 14
  5 --- 15
  6 --- 16
  6 --- 27
  6 ---- 28
  6 --- 32
  7 --- 17
  7 --- 18
  7 --- 19
  7 --- 20
  7 --- 25
  7 ---- 29
  7 --- 32
  8 --- 21
  8 --- 22
  8 --- 23
  8 --- 24
  8 --- 26
  8 ---- 30
  8 --- 31
  16 --- 36
  16 x--> 43
  16 --- 49
  16 --- 56
  17 --- 35
  17 x--> 44
  17 --- 48
  17 --- 54
  18 --- 34
  18 x--> 44
  18 --- 47
  18 --- 55
  19 --- 33
  19 x--> 44
  19 --- 46
  19 --- 53
  21 --- 37
  21 x--> 45
  21 --- 50
  21 --- 57
  22 --- 38
  22 x--> 45
  22 --- 52
  22 --- 59
  23 --- 39
  23 x--> 45
  23 --- 51
  23 --- 58
  28 --- 36
  28 --- 40
  28 --- 43
  28 --- 49
  28 --- 56
  29 --- 33
  29 --- 34
  29 --- 35
  29 --- 41
  29 --- 44
  29 --- 46
  29 --- 47
  29 --- 48
  29 --- 53
  29 --- 54
  29 --- 55
  30 --- 37
  30 --- 38
  30 --- 39
  30 --- 42
  30 --- 45
  30 --- 50
  30 --- 51
  30 --- 52
  30 --- 57
  30 --- 58
  30 --- 59
  32 --- 31
  33 --- 46
  33 --- 53
  55 <--x 33
  34 --- 47
  54 <--x 34
  34 --- 55
  35 --- 48
  53 <--x 35
  35 --- 54
  36 --- 49
  36 --- 56
  37 --- 50
  37 --- 57
  58 <--x 37
  38 --- 52
  57 <--x 38
  38 --- 59
  39 --- 51
  39 --- 58
  59 <--x 39
  49 <--x 40
  46 <--x 41
  47 <--x 41
  48 <--x 41
  50 <--x 42
  51 <--x 42
  52 <--x 42
```
