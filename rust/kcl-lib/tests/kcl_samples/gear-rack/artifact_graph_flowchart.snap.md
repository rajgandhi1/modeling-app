```mermaid
flowchart LR
  subgraph path5 [Path]
    5["Path<br>[603, 638, 0]"]
    9["Segment<br>[644, 667, 0]"]
    10["Segment<br>[673, 699, 0]"]
    11["Segment<br>[705, 729, 0]"]
    12["Segment<br>[735, 742, 0]"]
    32[Solid2d]
  end
  subgraph path6 [Path]
    6["Path<br>[877, 931, 0]"]
    13["Segment<br>[939, 980, 0]"]
    14["Segment<br>[988, 1020, 0]"]
    15["Segment<br>[1028, 1069, 0]"]
    16["Segment<br>[1077, 1102, 0]"]
    17["Segment<br>[1110, 1152, 0]"]
    18["Segment<br>[1160, 1193, 0]"]
    19["Segment<br>[1201, 1243, 0]"]
    20["Segment<br>[1251, 1258, 0]"]
    30[Solid2d]
  end
  subgraph path7 [Path]
    7["Path<br>[1571, 1614, 0]"]
    21["Segment<br>[1620, 1653, 0]"]
    22["Segment<br>[1659, 1701, 0]"]
    23["Segment<br>[1707, 1751, 0]"]
    24["Segment<br>[1757, 1764, 0]"]
    31[Solid2d]
  end
  subgraph path8 [Path]
    8["Path<br>[1899, 1941, 0]"]
    25["Segment<br>[1947, 1981, 0]"]
    26["Segment<br>[1987, 2030, 0]"]
    27["Segment<br>[2036, 2079, 0]"]
    28["Segment<br>[2085, 2092, 0]"]
    29[Solid2d]
  end
  1["Plane<br>[580, 597, 0]"]
  2["Plane<br>[852, 869, 0]"]
  3["Plane<br>[1548, 1565, 0]"]
  4["Plane<br>[1876, 1893, 0]"]
  33["Sweep Extrusion<br>[748, 771, 0]"]
  34["Sweep Extrusion<br>[1266, 1289, 0]"]
  35["Sweep Extrusion<br>[1770, 1793, 0]"]
  36["Sweep Extrusion<br>[2098, 2121, 0]"]
  37[Wall]
  38[Wall]
  39[Wall]
  40[Wall]
  41[Wall]
  42[Wall]
  43[Wall]
  44[Wall]
  45[Wall]
  46[Wall]
  47[Wall]
  48[Wall]
  49[Wall]
  50[Wall]
  51[Wall]
  52[Wall]
  53[Wall]
  54[Wall]
  55[Wall]
  56[Wall]
  57["Cap Start"]
  58["Cap Start"]
  59["Cap Start"]
  60["Cap Start"]
  61["Cap End"]
  62["Cap End"]
  63["Cap End"]
  64["Cap End"]
  65["SweepEdge Opposite"]
  66["SweepEdge Opposite"]
  67["SweepEdge Opposite"]
  68["SweepEdge Opposite"]
  69["SweepEdge Opposite"]
  70["SweepEdge Opposite"]
  71["SweepEdge Opposite"]
  72["SweepEdge Opposite"]
  73["SweepEdge Opposite"]
  74["SweepEdge Opposite"]
  75["SweepEdge Opposite"]
  76["SweepEdge Opposite"]
  77["SweepEdge Opposite"]
  78["SweepEdge Opposite"]
  79["SweepEdge Opposite"]
  80["SweepEdge Opposite"]
  81["SweepEdge Opposite"]
  82["SweepEdge Opposite"]
  83["SweepEdge Opposite"]
  84["SweepEdge Opposite"]
  85["SweepEdge Adjacent"]
  86["SweepEdge Adjacent"]
  87["SweepEdge Adjacent"]
  88["SweepEdge Adjacent"]
  89["SweepEdge Adjacent"]
  90["SweepEdge Adjacent"]
  91["SweepEdge Adjacent"]
  92["SweepEdge Adjacent"]
  93["SweepEdge Adjacent"]
  94["SweepEdge Adjacent"]
  95["SweepEdge Adjacent"]
  96["SweepEdge Adjacent"]
  97["SweepEdge Adjacent"]
  98["SweepEdge Adjacent"]
  99["SweepEdge Adjacent"]
  100["SweepEdge Adjacent"]
  101["SweepEdge Adjacent"]
  102["SweepEdge Adjacent"]
  103["SweepEdge Adjacent"]
  104["SweepEdge Adjacent"]
  1 --- 5
  2 --- 6
  3 --- 7
  4 --- 8
  5 --- 9
  5 --- 10
  5 --- 11
  5 --- 12
  5 --- 32
  5 ---- 33
  6 --- 13
  6 --- 14
  6 --- 15
  6 --- 16
  6 --- 17
  6 --- 18
  6 --- 19
  6 --- 20
  6 --- 30
  6 ---- 34
  7 --- 21
  7 --- 22
  7 --- 23
  7 --- 24
  7 --- 31
  7 ---- 35
  8 --- 25
  8 --- 26
  8 --- 27
  8 --- 28
  8 --- 29
  8 ---- 36
  9 --- 40
  9 x--> 60
  9 --- 68
  9 --- 85
  10 --- 38
  10 x--> 60
  10 --- 66
  10 --- 88
  11 --- 37
  11 x--> 60
  11 --- 65
  11 --- 86
  12 --- 39
  12 x--> 60
  12 --- 67
  12 --- 87
  13 --- 55
  13 x--> 58
  13 --- 82
  13 --- 103
  14 --- 56
  14 x--> 58
  14 --- 80
  14 --- 99
  15 --- 53
  15 x--> 58
  15 --- 84
  15 --- 104
  16 --- 54
  16 x--> 58
  16 --- 83
  16 --- 97
  17 --- 52
  17 x--> 58
  17 --- 79
  17 --- 98
  18 --- 50
  18 x--> 58
  18 --- 77
  18 --- 102
  19 --- 51
  19 x--> 58
  19 --- 81
  19 --- 100
  20 --- 49
  20 x--> 58
  20 --- 78
  20 --- 101
  21 --- 41
  21 x--> 59
  21 --- 70
  21 --- 91
  22 --- 43
  22 x--> 59
  22 --- 71
  22 --- 89
  23 --- 44
  23 x--> 59
  23 --- 72
  23 --- 92
  24 --- 42
  24 x--> 59
  24 --- 69
  24 --- 90
  25 --- 46
  25 x--> 57
  25 --- 76
  25 --- 95
  26 --- 45
  26 x--> 57
  26 --- 73
  26 --- 96
  27 --- 47
  27 x--> 57
  27 --- 75
  27 --- 93
  28 --- 48
  28 x--> 57
  28 --- 74
  28 --- 94
  33 --- 37
  33 --- 38
  33 --- 39
  33 --- 40
  33 --- 60
  33 --- 64
  33 --- 65
  33 --- 66
  33 --- 67
  33 --- 68
  33 --- 85
  33 --- 86
  33 --- 87
  33 --- 88
  34 --- 49
  34 --- 50
  34 --- 51
  34 --- 52
  34 --- 53
  34 --- 54
  34 --- 55
  34 --- 56
  34 --- 58
  34 --- 62
  34 --- 77
  34 --- 78
  34 --- 79
  34 --- 80
  34 --- 81
  34 --- 82
  34 --- 83
  34 --- 84
  34 --- 97
  34 --- 98
  34 --- 99
  34 --- 100
  34 --- 101
  34 --- 102
  34 --- 103
  34 --- 104
  35 --- 41
  35 --- 42
  35 --- 43
  35 --- 44
  35 --- 59
  35 --- 63
  35 --- 69
  35 --- 70
  35 --- 71
  35 --- 72
  35 --- 89
  35 --- 90
  35 --- 91
  35 --- 92
  36 --- 45
  36 --- 46
  36 --- 47
  36 --- 48
  36 --- 57
  36 --- 61
  36 --- 73
  36 --- 74
  36 --- 75
  36 --- 76
  36 --- 93
  36 --- 94
  36 --- 95
  36 --- 96
  37 --- 65
  37 --- 86
  88 <--x 37
  38 --- 66
  85 <--x 38
  38 --- 88
  39 --- 67
  86 <--x 39
  39 --- 87
  40 --- 68
  40 --- 85
  87 <--x 40
  41 --- 70
  90 <--x 41
  41 --- 91
  42 --- 69
  42 --- 90
  92 <--x 42
  43 --- 71
  43 --- 89
  91 <--x 43
  44 --- 72
  89 <--x 44
  44 --- 92
  45 --- 73
  95 <--x 45
  45 --- 96
  46 --- 76
  94 <--x 46
  46 --- 95
  47 --- 75
  47 --- 93
  96 <--x 47
  48 --- 74
  93 <--x 48
  48 --- 94
  49 --- 78
  100 <--x 49
  49 --- 101
  50 --- 77
  98 <--x 50
  50 --- 102
  51 --- 81
  51 --- 100
  102 <--x 51
  52 --- 79
  97 <--x 52
  52 --- 98
  53 --- 84
  99 <--x 53
  53 --- 104
  54 --- 83
  54 --- 97
  104 <--x 54
  55 --- 82
  101 <--x 55
  55 --- 103
  56 --- 80
  56 --- 99
  103 <--x 56
  73 <--x 61
  74 <--x 61
  75 <--x 61
  76 <--x 61
  77 <--x 62
  78 <--x 62
  79 <--x 62
  80 <--x 62
  81 <--x 62
  82 <--x 62
  83 <--x 62
  84 <--x 62
  69 <--x 63
  70 <--x 63
  71 <--x 63
  72 <--x 63
  65 <--x 64
  66 <--x 64
  67 <--x 64
  68 <--x 64
```
