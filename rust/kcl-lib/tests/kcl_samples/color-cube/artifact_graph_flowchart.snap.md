```mermaid
flowchart LR
  subgraph path7 [Path]
    7["Path<br>[791, 831, 0]"]
    13["Segment<br>[839, 904, 0]"]
    21["Segment<br>[912, 1009, 0]"]
    26["Segment<br>[1017, 1134, 0]"]
    33["Segment<br>[1142, 1198, 0]"]
    38["Segment<br>[1206, 1213, 0]"]
    43[Solid2d]
  end
  subgraph path8 [Path]
    8["Path<br>[791, 831, 0]"]
    14["Segment<br>[839, 904, 0]"]
    20["Segment<br>[912, 1009, 0]"]
    30["Segment<br>[1017, 1134, 0]"]
    32["Segment<br>[1142, 1198, 0]"]
    42["Segment<br>[1206, 1213, 0]"]
    44[Solid2d]
  end
  subgraph path9 [Path]
    9["Path<br>[791, 831, 0]"]
    15["Segment<br>[839, 904, 0]"]
    19["Segment<br>[912, 1009, 0]"]
    27["Segment<br>[1017, 1134, 0]"]
    35["Segment<br>[1142, 1198, 0]"]
    37["Segment<br>[1206, 1213, 0]"]
    45[Solid2d]
  end
  subgraph path10 [Path]
    10["Path<br>[791, 831, 0]"]
    18["Segment<br>[839, 904, 0]"]
    24["Segment<br>[912, 1009, 0]"]
    28["Segment<br>[1017, 1134, 0]"]
    34["Segment<br>[1142, 1198, 0]"]
    40["Segment<br>[1206, 1213, 0]"]
    46[Solid2d]
  end
  subgraph path11 [Path]
    11["Path<br>[791, 831, 0]"]
    16["Segment<br>[839, 904, 0]"]
    23["Segment<br>[912, 1009, 0]"]
    25["Segment<br>[1017, 1134, 0]"]
    31["Segment<br>[1142, 1198, 0]"]
    39["Segment<br>[1206, 1213, 0]"]
    47[Solid2d]
  end
  subgraph path12 [Path]
    12["Path<br>[791, 831, 0]"]
    17["Segment<br>[839, 904, 0]"]
    22["Segment<br>[912, 1009, 0]"]
    29["Segment<br>[1017, 1134, 0]"]
    36["Segment<br>[1142, 1198, 0]"]
    41["Segment<br>[1206, 1213, 0]"]
    48[Solid2d]
  end
  1["Plane<br>[374, 408, 0]"]
  2["Plane<br>[423, 458, 0]"]
  3["Plane<br>[472, 507, 0]"]
  4["Plane<br>[522, 558, 0]"]
  5["Plane<br>[570, 620, 0]"]
  6["Plane<br>[633, 668, 0]"]
  49["Sweep Extrusion<br>[1221, 1252, 0]"]
  50["Sweep Extrusion<br>[1221, 1252, 0]"]
  51["Sweep Extrusion<br>[1221, 1252, 0]"]
  52["Sweep Extrusion<br>[1221, 1252, 0]"]
  53["Sweep Extrusion<br>[1221, 1252, 0]"]
  54["Sweep Extrusion<br>[1221, 1252, 0]"]
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
  78[Wall]
  79["Cap Start"]
  80["Cap Start"]
  81["Cap Start"]
  82["Cap Start"]
  83["Cap Start"]
  84["Cap Start"]
  85["Cap End"]
  86["Cap End"]
  87["Cap End"]
  88["Cap End"]
  89["Cap End"]
  90["Cap End"]
  91["SweepEdge Opposite"]
  92["SweepEdge Opposite"]
  93["SweepEdge Opposite"]
  94["SweepEdge Opposite"]
  95["SweepEdge Opposite"]
  96["SweepEdge Opposite"]
  97["SweepEdge Opposite"]
  98["SweepEdge Opposite"]
  99["SweepEdge Opposite"]
  100["SweepEdge Opposite"]
  101["SweepEdge Opposite"]
  102["SweepEdge Opposite"]
  103["SweepEdge Opposite"]
  104["SweepEdge Opposite"]
  105["SweepEdge Opposite"]
  106["SweepEdge Opposite"]
  107["SweepEdge Opposite"]
  108["SweepEdge Opposite"]
  109["SweepEdge Opposite"]
  110["SweepEdge Opposite"]
  111["SweepEdge Opposite"]
  112["SweepEdge Opposite"]
  113["SweepEdge Opposite"]
  114["SweepEdge Opposite"]
  115["SweepEdge Adjacent"]
  116["SweepEdge Adjacent"]
  117["SweepEdge Adjacent"]
  118["SweepEdge Adjacent"]
  119["SweepEdge Adjacent"]
  120["SweepEdge Adjacent"]
  121["SweepEdge Adjacent"]
  122["SweepEdge Adjacent"]
  123["SweepEdge Adjacent"]
  124["SweepEdge Adjacent"]
  125["SweepEdge Adjacent"]
  126["SweepEdge Adjacent"]
  127["SweepEdge Adjacent"]
  128["SweepEdge Adjacent"]
  129["SweepEdge Adjacent"]
  130["SweepEdge Adjacent"]
  131["SweepEdge Adjacent"]
  132["SweepEdge Adjacent"]
  133["SweepEdge Adjacent"]
  134["SweepEdge Adjacent"]
  135["SweepEdge Adjacent"]
  136["SweepEdge Adjacent"]
  137["SweepEdge Adjacent"]
  138["SweepEdge Adjacent"]
  1 --- 8
  2 --- 10
  3 --- 12
  4 --- 11
  5 --- 7
  6 --- 9
  7 --- 13
  7 --- 21
  7 --- 26
  7 --- 33
  7 --- 38
  7 --- 43
  7 ---- 51
  8 --- 14
  8 --- 20
  8 --- 30
  8 --- 32
  8 --- 42
  8 --- 44
  8 ---- 49
  9 --- 15
  9 --- 19
  9 --- 27
  9 --- 35
  9 --- 37
  9 --- 45
  9 ---- 52
  10 --- 18
  10 --- 24
  10 --- 28
  10 --- 34
  10 --- 40
  10 --- 46
  10 ---- 53
  11 --- 16
  11 --- 23
  11 --- 25
  11 --- 31
  11 --- 39
  11 --- 47
  11 ---- 50
  12 --- 17
  12 --- 22
  12 --- 29
  12 --- 36
  12 --- 41
  12 --- 48
  12 ---- 54
  13 --- 65
  13 x--> 81
  13 --- 101
  13 --- 123
  14 --- 57
  14 x--> 83
  14 --- 92
  14 --- 118
  15 --- 69
  15 x--> 84
  15 --- 103
  15 --- 127
  16 --- 61
  16 x--> 80
  16 --- 97
  16 --- 121
  17 --- 76
  17 x--> 79
  17 --- 111
  17 --- 138
  18 --- 73
  18 x--> 82
  18 --- 107
  18 --- 133
  19 --- 67
  19 x--> 84
  19 --- 105
  19 --- 128
  20 --- 55
  20 x--> 83
  20 --- 94
  20 --- 116
  21 --- 64
  21 x--> 81
  21 --- 100
  21 --- 124
  22 --- 75
  22 x--> 79
  22 --- 113
  22 --- 137
  23 --- 62
  23 x--> 80
  23 --- 96
  23 --- 120
  24 --- 74
  24 x--> 82
  24 --- 109
  24 --- 132
  25 --- 60
  25 x--> 80
  25 --- 98
  25 --- 119
  26 --- 63
  26 x--> 81
  26 --- 99
  26 --- 126
  27 --- 68
  27 x--> 84
  27 --- 106
  27 --- 129
  28 --- 72
  28 x--> 82
  28 --- 108
  28 --- 134
  29 --- 77
  29 x--> 79
  29 --- 114
  29 --- 136
  30 --- 58
  30 x--> 83
  30 --- 91
  30 --- 117
  31 --- 59
  31 x--> 80
  31 --- 95
  31 --- 122
  32 --- 56
  32 x--> 83
  32 --- 93
  32 --- 115
  33 --- 66
  33 x--> 81
  33 --- 102
  33 --- 125
  34 --- 71
  34 x--> 82
  34 --- 110
  34 --- 131
  35 --- 70
  35 x--> 84
  35 --- 104
  35 --- 130
  36 --- 78
  36 x--> 79
  36 --- 112
  36 --- 135
  49 --- 55
  49 --- 56
  49 --- 57
  49 --- 58
  49 --- 83
  49 --- 89
  49 --- 91
  49 --- 92
  49 --- 93
  49 --- 94
  49 --- 115
  49 --- 116
  49 --- 117
  49 --- 118
  50 --- 59
  50 --- 60
  50 --- 61
  50 --- 62
  50 --- 80
  50 --- 86
  50 --- 95
  50 --- 96
  50 --- 97
  50 --- 98
  50 --- 119
  50 --- 120
  50 --- 121
  50 --- 122
  51 --- 63
  51 --- 64
  51 --- 65
  51 --- 66
  51 --- 81
  51 --- 87
  51 --- 99
  51 --- 100
  51 --- 101
  51 --- 102
  51 --- 123
  51 --- 124
  51 --- 125
  51 --- 126
  52 --- 67
  52 --- 68
  52 --- 69
  52 --- 70
  52 --- 84
  52 --- 90
  52 --- 103
  52 --- 104
  52 --- 105
  52 --- 106
  52 --- 127
  52 --- 128
  52 --- 129
  52 --- 130
  53 --- 71
  53 --- 72
  53 --- 73
  53 --- 74
  53 --- 82
  53 --- 88
  53 --- 107
  53 --- 108
  53 --- 109
  53 --- 110
  53 --- 131
  53 --- 132
  53 --- 133
  53 --- 134
  54 --- 75
  54 --- 76
  54 --- 77
  54 --- 78
  54 --- 79
  54 --- 85
  54 --- 111
  54 --- 112
  54 --- 113
  54 --- 114
  54 --- 135
  54 --- 136
  54 --- 137
  54 --- 138
  55 --- 94
  55 --- 116
  118 <--x 55
  56 --- 93
  56 --- 115
  117 <--x 56
  57 --- 92
  115 <--x 57
  57 --- 118
  58 --- 91
  116 <--x 58
  58 --- 117
  59 --- 95
  119 <--x 59
  59 --- 122
  60 --- 98
  60 --- 119
  120 <--x 60
  61 --- 97
  61 --- 121
  122 <--x 61
  62 --- 96
  62 --- 120
  121 <--x 62
  63 --- 99
  124 <--x 63
  63 --- 126
  64 --- 100
  123 <--x 64
  64 --- 124
  65 --- 101
  65 --- 123
  125 <--x 65
  66 --- 102
  66 --- 125
  126 <--x 66
  67 --- 105
  127 <--x 67
  67 --- 128
  68 --- 106
  128 <--x 68
  68 --- 129
  69 --- 103
  69 --- 127
  130 <--x 69
  70 --- 104
  129 <--x 70
  70 --- 130
  71 --- 110
  71 --- 131
  134 <--x 71
  72 --- 108
  132 <--x 72
  72 --- 134
  73 --- 107
  131 <--x 73
  73 --- 133
  74 --- 109
  74 --- 132
  133 <--x 74
  75 --- 113
  75 --- 137
  138 <--x 75
  76 --- 111
  135 <--x 76
  76 --- 138
  77 --- 114
  77 --- 136
  137 <--x 77
  78 --- 112
  78 --- 135
  136 <--x 78
  111 <--x 85
  112 <--x 85
  113 <--x 85
  114 <--x 85
  95 <--x 86
  96 <--x 86
  97 <--x 86
  98 <--x 86
  99 <--x 87
  100 <--x 87
  101 <--x 87
  102 <--x 87
  107 <--x 88
  108 <--x 88
  109 <--x 88
  110 <--x 88
  91 <--x 89
  92 <--x 89
  93 <--x 89
  94 <--x 89
  103 <--x 90
  104 <--x 90
  105 <--x 90
  106 <--x 90
```
