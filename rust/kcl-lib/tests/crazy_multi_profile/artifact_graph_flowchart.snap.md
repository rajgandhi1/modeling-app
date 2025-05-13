```mermaid
flowchart LR
  subgraph path4 [Path]
    4["Path<br>[43, 86, 0]"]
    15["Segment<br>[92, 130, 0]"]
    16["Segment<br>[136, 175, 0]"]
    17["Segment<br>[181, 237, 0]"]
    18["Segment<br>[243, 250, 0]"]
    58[Solid2d]
  end
  subgraph path5 [Path]
    5["Path<br>[362, 405, 0]"]
    19["Segment<br>[411, 435, 0]"]
    20["Segment<br>[441, 466, 0]"]
  end
  subgraph path6 [Path]
    6["Path<br>[480, 522, 0]"]
    21["Segment<br>[528, 593, 0]"]
    22["Segment<br>[599, 667, 0]"]
    23["Segment<br>[673, 761, 0]"]
    24["Segment<br>[767, 823, 0]"]
    25["Segment<br>[829, 836, 0]"]
    57[Solid2d]
  end
  subgraph path7 [Path]
    7["Path<br>[850, 892, 0]"]
    26["Segment<br>[898, 918, 0]"]
    27["Segment<br>[924, 950, 0]"]
    28["Segment<br>[956, 1012, 0]"]
    29["Segment<br>[1018, 1025, 0]"]
    54[Solid2d]
  end
  subgraph path8 [Path]
    8["Path<br>[1039, 1094, 0]"]
    30["Segment<br>[1039, 1094, 0]"]
    59[Solid2d]
  end
  subgraph path9 [Path]
    9["Path<br>[1108, 1150, 0]"]
    31["Segment<br>[1156, 1180, 0]"]
    32["Segment<br>[1186, 1211, 0]"]
    33["Segment<br>[1217, 1273, 0]"]
    34["Segment<br>[1279, 1286, 0]"]
    56[Solid2d]
  end
  subgraph path10 [Path]
    10["Path<br>[1456, 1497, 0]"]
    35["Segment<br>[1503, 1527, 0]"]
    36["Segment<br>[1533, 1558, 0]"]
  end
  subgraph path11 [Path]
    11["Path<br>[1572, 1614, 0]"]
    37["Segment<br>[1620, 1644, 0]"]
    38["Segment<br>[1650, 1675, 0]"]
    39["Segment<br>[1681, 1737, 0]"]
    40["Segment<br>[1743, 1750, 0]"]
    53[Solid2d]
  end
  subgraph path12 [Path]
    12["Path<br>[1764, 1806, 0]"]
    41["Segment<br>[1812, 1835, 0]"]
    42["Segment<br>[1841, 1866, 0]"]
    43["Segment<br>[1872, 1928, 0]"]
    44["Segment<br>[1934, 1941, 0]"]
    52[Solid2d]
  end
  subgraph path13 [Path]
    13["Path<br>[1955, 2011, 0]"]
    45["Segment<br>[1955, 2011, 0]"]
    51[Solid2d]
  end
  subgraph path14 [Path]
    14["Path<br>[2025, 2068, 0]"]
    46["Segment<br>[2074, 2139, 0]"]
    47["Segment<br>[2145, 2213, 0]"]
    48["Segment<br>[2219, 2307, 0]"]
    49["Segment<br>[2313, 2369, 0]"]
    50["Segment<br>[2375, 2382, 0]"]
    55[Solid2d]
  end
  1["Plane<br>[12, 29, 0]"]
  2["Plane<br>[1424, 1442, 0]"]
  3["StartSketchOnFace<br>[309, 348, 0]"]
  60["Sweep Extrusion<br>[264, 296, 0]"]
  61["Sweep RevolveAboutEdge<br>[1300, 1366, 0]"]
  62["Sweep Extrusion<br>[1380, 1411, 0]"]
  63["Sweep Extrusion<br>[2396, 2429, 0]"]
  64["Sweep RevolveAboutEdge<br>[2443, 2488, 0]"]
  65[Wall]
  66[Wall]
  67[Wall]
  68[Wall]
  69[Wall]
  70[Wall]
  71[Wall]
  72[Wall]
  73[Wall]
  74[Wall]
  75[Wall]
  76[Wall]
  77[Wall]
  78["Cap Start"]
  79["Cap Start"]
  80["Cap Start"]
  81["Cap Start"]
  82["Cap End"]
  83["Cap End"]
  84["Cap End"]
  85["Cap End"]
  86["SweepEdge Opposite"]
  87["SweepEdge Opposite"]
  88["SweepEdge Opposite"]
  89["SweepEdge Opposite"]
  90["SweepEdge Opposite"]
  91["SweepEdge Opposite"]
  92["SweepEdge Opposite"]
  93["SweepEdge Opposite"]
  94["SweepEdge Opposite"]
  95["SweepEdge Opposite"]
  96["SweepEdge Opposite"]
  97["SweepEdge Opposite"]
  98["SweepEdge Opposite"]
  99["SweepEdge Adjacent"]
  100["SweepEdge Adjacent"]
  101["SweepEdge Adjacent"]
  102["SweepEdge Adjacent"]
  103["SweepEdge Adjacent"]
  104["SweepEdge Adjacent"]
  105["SweepEdge Adjacent"]
  106["SweepEdge Adjacent"]
  107["SweepEdge Adjacent"]
  108["SweepEdge Adjacent"]
  109["SweepEdge Adjacent"]
  110["SweepEdge Adjacent"]
  111["SweepEdge Adjacent"]
  1 --- 4
  2 --- 10
  2 --- 11
  2 --- 12
  2 --- 13
  2 --- 14
  71 x--> 3
  4 --- 15
  4 --- 16
  4 --- 17
  4 --- 18
  4 --- 58
  4 ---- 60
  5 --- 19
  5 --- 20
  71 --- 5
  6 --- 21
  6 --- 22
  6 --- 23
  6 --- 24
  6 --- 25
  6 --- 57
  71 --- 6
  7 --- 26
  7 --- 27
  7 --- 28
  7 --- 29
  7 --- 54
  7 ---- 61
  71 --- 7
  8 --- 30
  8 --- 59
  71 --- 8
  9 --- 31
  9 --- 32
  9 --- 33
  9 --- 34
  9 --- 56
  9 ---- 62
  71 --- 9
  10 --- 35
  10 --- 36
  11 --- 37
  11 --- 38
  11 --- 39
  11 --- 40
  11 --- 53
  11 ---- 64
  12 --- 41
  12 --- 42
  12 --- 43
  12 --- 44
  12 --- 52
  13 --- 45
  13 --- 51
  14 --- 46
  14 --- 47
  14 --- 48
  14 --- 49
  14 --- 50
  14 --- 55
  14 ---- 63
  15 --- 70
  15 x--> 81
  15 --- 91
  15 --- 105
  16 --- 71
  16 x--> 81
  16 --- 90
  16 --- 104
  17 --- 69
  17 x--> 81
  17 --- 92
  17 --- 103
  31 --- 77
  31 x--> 80
  31 --- 98
  31 --- 110
  32 --- 75
  32 x--> 80
  32 --- 96
  32 --- 111
  33 --- 76
  33 x--> 80
  33 --- 97
  33 --- 109
  37 --- 74
  37 x--> 79
  37 --- 94
  37 --- 108
  38 --- 73
  38 x--> 79
  38 --- 95
  38 --- 107
  39 --- 72
  39 x--> 79
  39 --- 93
  39 --- 106
  46 --- 67
  46 x--> 78
  46 --- 86
  46 --- 101
  47 --- 65
  47 x--> 78
  47 --- 89
  47 --- 100
  48 --- 66
  48 x--> 78
  48 --- 87
  48 --- 102
  49 --- 68
  49 x--> 78
  49 --- 88
  49 --- 99
  60 --- 69
  60 --- 70
  60 --- 71
  60 --- 81
  60 --- 85
  60 --- 90
  60 --- 91
  60 --- 92
  60 --- 103
  60 --- 104
  60 --- 105
  62 --- 75
  62 --- 76
  62 --- 77
  62 --- 80
  62 --- 84
  62 --- 96
  62 --- 97
  62 --- 98
  62 --- 109
  62 --- 110
  62 --- 111
  63 --- 65
  63 --- 66
  63 --- 67
  63 --- 68
  63 --- 78
  63 --- 82
  63 --- 86
  63 --- 87
  63 --- 88
  63 --- 89
  63 --- 99
  63 --- 100
  63 --- 101
  63 --- 102
  64 --- 72
  64 --- 73
  64 --- 74
  64 --- 79
  64 --- 83
  64 --- 93
  64 --- 94
  64 --- 95
  64 --- 106
  64 --- 107
  64 --- 108
  65 --- 89
  65 --- 100
  101 <--x 65
  66 --- 87
  100 <--x 66
  66 --- 102
  67 --- 86
  99 <--x 67
  67 --- 101
  68 --- 88
  68 --- 99
  102 <--x 68
  69 --- 92
  69 --- 103
  104 <--x 69
  70 --- 91
  103 <--x 70
  70 --- 105
  71 --- 90
  71 --- 104
  105 <--x 71
  72 --- 93
  72 --- 106
  107 <--x 72
  73 --- 95
  73 --- 107
  108 <--x 73
  74 --- 94
  106 <--x 74
  74 --- 108
  75 --- 96
  110 <--x 75
  75 --- 111
  76 --- 97
  76 --- 109
  111 <--x 76
  77 --- 98
  109 <--x 77
  77 --- 110
  86 <--x 82
  87 <--x 82
  88 <--x 82
  89 <--x 82
  93 <--x 83
  94 <--x 83
  95 <--x 83
  96 <--x 84
  97 <--x 84
  98 <--x 84
  90 <--x 85
  91 <--x 85
  92 <--x 85
```
