```mermaid
flowchart LR
  subgraph path9 [Path]
    9["Path<br>[947, 993, 0]"]
    14["Segment<br>[1001, 1023, 0]"]
    17["Segment<br>[1031, 1061, 0]"]
    19["Segment<br>[1069, 1113, 0]"]
    20["Segment<br>[1121, 1148, 0]"]
    22["Segment<br>[1156, 1200, 0]"]
    24["Segment<br>[1208, 1215, 0]"]
    35[Solid2d]
  end
  subgraph path10 [Path]
    10["Path<br>[947, 993, 0]"]
    15["Segment<br>[1001, 1023, 0]"]
    16["Segment<br>[1031, 1061, 0]"]
    18["Segment<br>[1069, 1113, 0]"]
    21["Segment<br>[1121, 1148, 0]"]
    23["Segment<br>[1156, 1200, 0]"]
    25["Segment<br>[1208, 1215, 0]"]
    38[Solid2d]
  end
  subgraph path11 [Path]
    11["Path<br>[2256, 2344, 0]"]
    26["Segment<br>[2350, 2414, 0]"]
    27["Segment<br>[2420, 2484, 0]"]
    28["Segment<br>[2490, 2543, 0]"]
    29["Segment<br>[2549, 2570, 0]"]
    37[Solid2d]
  end
  subgraph path12 [Path]
    12["Path<br>[2901, 3067, 0]"]
    30["Segment<br>[2901, 3067, 0]"]
    36[Solid2d]
  end
  subgraph path13 [Path]
    13["Path<br>[4380, 4405, 0]"]
    31["Segment<br>[4411, 4483, 0]"]
    32["Segment<br>[4489, 4562, 0]"]
    33["Segment<br>[4568, 4621, 0]"]
    34["Segment<br>[4627, 4648, 0]"]
    39[Solid2d]
  end
  1["Plane<br>[1301, 1348, 0]"]
  2["Plane<br>[1880, 1927, 0]"]
  3["Plane<br>[2233, 2250, 0]"]
  4["Plane<br>[4341, 4373, 0]"]
  5["StartSketchOnPlane<br>[919, 939, 0]"]
  6["StartSketchOnPlane<br>[919, 939, 0]"]
  7["StartSketchOnPlane<br>[4327, 4374, 0]"]
  8["StartSketchOnFace<br>[2853, 2895, 0]"]
  40["Sweep Extrusion<br>[1288, 1391, 0]"]
  41["Sweep Revolve<br>[1867, 1958, 0]"]
  42["Sweep Extrusion<br>[2576, 2600, 0]"]
  43["Sweep Extrusion<br>[3289, 3316, 0]"]
  44["Sweep Extrusion<br>[3289, 3316, 0]"]
  45["Sweep Extrusion<br>[3289, 3316, 0]"]
  46["Sweep Extrusion<br>[3289, 3316, 0]"]
  47["Sweep Extrusion<br>[4654, 4698, 0]"]
  48[Wall]
  49[Wall]
  50[Wall]
  51[Wall]
  52[Wall]
  53[Wall]
  54[Wall]
  55[Wall]
  56[Wall]
  57[Wall]
  58[Wall]
  59[Wall]
  60[Wall]
  61[Wall]
  62[Wall]
  63[Wall]
  64[Wall]
  65[Wall]
  66[Wall]
  67["Cap Start"]
  68["Cap Start"]
  69["Cap Start"]
  70["Cap Start"]
  71["Cap Start"]
  72["Cap End"]
  73["Cap End"]
  74["Cap End"]
  75["Cap End"]
  76["SweepEdge Opposite"]
  77["SweepEdge Opposite"]
  78["SweepEdge Opposite"]
  79["SweepEdge Opposite"]
  80["SweepEdge Opposite"]
  81["SweepEdge Opposite"]
  82["SweepEdge Opposite"]
  83["SweepEdge Opposite"]
  84["SweepEdge Opposite"]
  85["SweepEdge Opposite"]
  86["SweepEdge Opposite"]
  87["SweepEdge Opposite"]
  88["SweepEdge Opposite"]
  89["SweepEdge Opposite"]
  90["SweepEdge Opposite"]
  91["SweepEdge Opposite"]
  92["SweepEdge Opposite"]
  93["SweepEdge Opposite"]
  94["SweepEdge Opposite"]
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
  105["SweepEdge Adjacent"]
  106["SweepEdge Adjacent"]
  107["SweepEdge Adjacent"]
  108["SweepEdge Adjacent"]
  109["SweepEdge Adjacent"]
  110["SweepEdge Adjacent"]
  111["SweepEdge Adjacent"]
  112["SweepEdge Adjacent"]
  113["SweepEdge Adjacent"]
  114["EdgeCut Fillet<br>[2606, 2836, 0]"]
  115["EdgeCut Fillet<br>[2606, 2836, 0]"]
  116["EdgeCut Fillet<br>[2606, 2836, 0]"]
  117["EdgeCut Fillet<br>[2606, 2836, 0]"]
  118["EdgeCut Fillet<br>[4704, 4937, 0]"]
  119["EdgeCut Fillet<br>[4704, 4937, 0]"]
  120["EdgeCut Fillet<br>[4704, 4937, 0]"]
  121["EdgeCut Fillet<br>[4704, 4937, 0]"]
  1 <--x 6
  1 --- 10
  2 <--x 5
  2 --- 9
  3 --- 11
  4 <--x 7
  4 --- 13
  70 x--> 8
  9 --- 14
  9 --- 17
  9 --- 19
  9 --- 20
  9 --- 22
  9 --- 24
  9 --- 35
  9 ---- 41
  10 --- 15
  10 --- 16
  10 --- 18
  10 --- 21
  10 --- 23
  10 --- 25
  10 --- 38
  10 ---- 40
  11 --- 26
  11 --- 27
  11 --- 28
  11 --- 29
  11 --- 37
  11 ---- 42
  12 --- 30
  12 --- 36
  12 ---- 43
  70 --- 12
  13 --- 31
  13 --- 32
  13 --- 33
  13 --- 34
  13 --- 39
  13 ---- 47
  14 --- 66
  14 x--> 67
  14 --- 94
  14 --- 109
  15 --- 50
  15 x--> 68
  15 --- 76
  15 --- 95
  16 --- 49
  16 x--> 68
  16 --- 80
  16 --- 97
  17 --- 64
  17 x--> 67
  17 --- 91
  17 --- 112
  18 --- 48
  18 x--> 68
  18 --- 79
  18 --- 96
  19 --- 63
  19 x--> 67
  19 --- 92
  19 --- 110
  20 --- 65
  20 x--> 67
  20 --- 90
  20 --- 113
  21 --- 51
  21 x--> 68
  21 --- 77
  21 --- 99
  22 --- 62
  22 x--> 67
  22 --- 93
  22 --- 111
  23 --- 52
  23 x--> 68
  23 --- 78
  23 --- 98
  26 --- 58
  26 x--> 70
  26 --- 87
  26 --- 108
  27 --- 59
  27 x--> 70
  27 --- 89
  27 --- 106
  28 --- 60
  28 x--> 70
  28 --- 88
  28 --- 107
  29 --- 61
  29 x--> 70
  29 --- 86
  29 --- 105
  30 --- 57
  30 x--> 70
  30 --- 85
  30 --- 104
  31 --- 56
  31 x--> 71
  31 --- 81
  31 --- 102
  32 --- 55
  32 x--> 71
  32 --- 84
  32 --- 101
  33 --- 53
  33 x--> 71
  33 --- 83
  33 --- 103
  34 --- 54
  34 x--> 71
  34 --- 82
  34 --- 100
  40 --- 48
  40 --- 49
  40 --- 50
  40 --- 51
  40 --- 52
  40 --- 68
  40 --- 73
  40 --- 76
  40 --- 77
  40 --- 78
  40 --- 79
  40 --- 80
  40 --- 95
  40 --- 96
  40 --- 97
  40 --- 98
  40 --- 99
  41 --- 62
  41 --- 63
  41 --- 64
  41 --- 65
  41 --- 66
  41 --- 67
  41 --- 72
  41 --- 90
  41 --- 91
  41 --- 92
  41 --- 93
  41 --- 94
  41 --- 109
  41 --- 110
  41 --- 111
  41 --- 112
  41 --- 113
  42 --- 58
  42 --- 59
  42 --- 60
  42 --- 61
  42 --- 70
  42 --- 74
  42 --- 86
  42 --- 87
  42 --- 88
  42 --- 89
  42 --- 105
  42 --- 106
  42 --- 107
  42 --- 108
  43 --- 57
  43 --- 69
  43 --- 85
  43 --- 104
  47 --- 53
  47 --- 54
  47 --- 55
  47 --- 56
  47 --- 71
  47 --- 75
  47 --- 81
  47 --- 82
  47 --- 83
  47 --- 84
  47 --- 100
  47 --- 101
  47 --- 102
  47 --- 103
  48 --- 79
  48 --- 96
  97 <--x 48
  49 --- 80
  95 <--x 49
  49 --- 97
  50 --- 76
  50 --- 95
  98 <--x 50
  51 --- 77
  96 <--x 51
  51 --- 99
  52 --- 78
  52 --- 98
  99 <--x 52
  53 --- 83
  101 <--x 53
  53 --- 103
  54 --- 82
  54 --- 100
  103 <--x 54
  55 --- 84
  55 --- 101
  102 <--x 55
  56 --- 81
  100 <--x 56
  56 --- 102
  57 --- 85
  57 --- 104
  58 --- 87
  105 <--x 58
  58 --- 108
  59 --- 89
  59 --- 106
  108 <--x 59
  60 --- 88
  106 <--x 60
  60 --- 107
  61 --- 86
  61 --- 105
  107 <--x 61
  62 --- 93
  62 --- 111
  113 <--x 62
  63 --- 92
  63 --- 110
  112 <--x 63
  64 --- 91
  109 <--x 64
  64 --- 112
  65 --- 90
  110 <--x 65
  65 --- 113
  66 --- 94
  66 --- 109
  111 <--x 66
  85 <--x 69
  90 <--x 72
  91 <--x 72
  92 <--x 72
  93 <--x 72
  94 <--x 72
  76 <--x 73
  77 <--x 73
  78 <--x 73
  79 <--x 73
  80 <--x 73
  86 <--x 74
  87 <--x 74
  88 <--x 74
  89 <--x 74
  81 <--x 75
  82 <--x 75
  83 <--x 75
  84 <--x 75
  100 <--x 121
  101 <--x 119
  102 <--x 120
  103 <--x 118
  105 <--x 115
  106 <--x 117
  107 <--x 116
  108 <--x 114
```
