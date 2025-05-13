```mermaid
flowchart LR
  subgraph path3 [Path]
    3["Path<br>[206, 250, 0]"]
    5["Segment<br>[256, 290, 0]"]
    6["Segment<br>[296, 365, 0]"]
    7["Segment<br>[371, 398, 0]"]
    8["Segment<br>[404, 435, 0]"]
    9["Segment<br>[441, 476, 0]"]
    10["Segment<br>[482, 562, 0]"]
    11["Segment<br>[568, 599, 0]"]
    12["Segment<br>[605, 664, 0]"]
    13["Segment<br>[670, 697, 0]"]
    14["Segment<br>[703, 725, 0]"]
    15["Segment<br>[731, 766, 0]"]
    16["Segment<br>[772, 818, 0]"]
    17["Segment<br>[824, 832, 0]"]
    31[Solid2d]
  end
  subgraph path4 [Path]
    4["Path<br>[1000, 1044, 0]"]
    18["Segment<br>[1050, 1084, 0]"]
    19["Segment<br>[1090, 1159, 0]"]
    20["Segment<br>[1165, 1192, 0]"]
    21["Segment<br>[1198, 1229, 0]"]
    22["Segment<br>[1235, 1270, 0]"]
    23["Segment<br>[1276, 1356, 0]"]
    24["Segment<br>[1362, 1393, 0]"]
    25["Segment<br>[1399, 1458, 0]"]
    26["Segment<br>[1464, 1491, 0]"]
    27["Segment<br>[1497, 1519, 0]"]
    28["Segment<br>[1525, 1560, 0]"]
    29["Segment<br>[1566, 1612, 0]"]
    30["Segment<br>[1618, 1626, 0]"]
    32[Solid2d]
  end
  1["Plane<br>[182, 200, 0]"]
  2["Plane<br>[976, 994, 0]"]
  33["Sweep Revolve<br>[843, 962, 0]"]
  34["Sweep Extrusion<br>[1632, 1670, 0]"]
  35[Wall]
  36[Wall]
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
  57[Wall]
  58[Wall]
  59[Wall]
  60[Wall]
  61["Cap Start"]
  62["Cap Start"]
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
  85["SweepEdge Opposite"]
  86["SweepEdge Opposite"]
  87["SweepEdge Opposite"]
  88["SweepEdge Opposite"]
  89["SweepEdge Opposite"]
  90["SweepEdge Opposite"]
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
  105["SweepEdge Adjacent"]
  106["SweepEdge Adjacent"]
  107["SweepEdge Adjacent"]
  108["SweepEdge Adjacent"]
  109["SweepEdge Adjacent"]
  110["SweepEdge Adjacent"]
  111["SweepEdge Adjacent"]
  112["SweepEdge Adjacent"]
  113["SweepEdge Adjacent"]
  114["SweepEdge Adjacent"]
  115["SweepEdge Adjacent"]
  116["SweepEdge Adjacent"]
  1 --- 3
  2 --- 4
  3 --- 5
  3 --- 6
  3 --- 7
  3 --- 8
  3 --- 9
  3 --- 10
  3 --- 11
  3 --- 12
  3 --- 13
  3 --- 14
  3 --- 15
  3 --- 16
  3 --- 17
  3 --- 31
  3 ---- 33
  4 --- 18
  4 --- 19
  4 --- 20
  4 --- 21
  4 --- 22
  4 --- 23
  4 --- 24
  4 --- 25
  4 --- 26
  4 --- 27
  4 --- 28
  4 --- 29
  4 --- 30
  4 --- 32
  4 ---- 34
  5 --- 41
  5 x--> 61
  5 --- 76
  5 --- 103
  6 --- 40
  6 x--> 61
  6 --- 72
  6 --- 101
  7 --- 42
  7 x--> 61
  7 --- 65
  7 --- 96
  8 --- 38
  8 x--> 61
  8 --- 69
  8 --- 98
  9 --- 37
  9 x--> 61
  9 --- 66
  9 --- 102
  10 --- 44
  10 x--> 61
  10 --- 75
  10 --- 95
  11 --- 46
  11 x--> 61
  11 --- 70
  11 --- 94
  12 --- 45
  12 x--> 61
  12 --- 71
  12 --- 99
  13 --- 35
  13 x--> 61
  13 --- 74
  13 --- 100
  14 --- 36
  14 x--> 61
  14 --- 67
  14 --- 92
  15 --- 47
  15 x--> 61
  15 --- 68
  15 --- 93
  16 --- 43
  16 x--> 61
  16 --- 77
  16 --- 97
  17 --- 39
  17 x--> 61
  17 --- 73
  17 --- 91
  18 --- 51
  18 x--> 62
  18 --- 84
  18 --- 106
  19 --- 54
  19 x--> 62
  19 --- 86
  19 --- 109
  20 --- 57
  20 x--> 62
  20 --- 89
  20 --- 108
  21 --- 60
  21 x--> 62
  21 --- 81
  21 --- 110
  22 --- 49
  22 x--> 62
  22 --- 82
  22 --- 111
  23 --- 58
  23 x--> 62
  23 --- 79
  23 --- 115
  24 --- 53
  24 x--> 62
  24 --- 87
  24 --- 112
  25 --- 52
  25 x--> 62
  25 --- 88
  25 --- 107
  26 --- 59
  26 x--> 62
  26 --- 85
  26 --- 113
  27 --- 56
  27 x--> 62
  27 --- 90
  27 --- 114
  28 --- 48
  28 x--> 62
  28 --- 80
  28 --- 116
  29 --- 50
  29 x--> 62
  29 --- 78
  29 --- 104
  30 --- 55
  30 x--> 62
  30 --- 83
  30 --- 105
  33 --- 35
  33 --- 36
  33 --- 37
  33 --- 38
  33 --- 39
  33 --- 40
  33 --- 41
  33 --- 42
  33 --- 43
  33 --- 44
  33 --- 45
  33 --- 46
  33 --- 47
  33 --- 61
  33 --- 63
  33 --- 65
  33 --- 66
  33 --- 67
  33 --- 68
  33 --- 69
  33 --- 70
  33 --- 71
  33 --- 72
  33 --- 73
  33 --- 74
  33 --- 75
  33 --- 76
  33 --- 77
  33 --- 91
  33 --- 92
  33 --- 93
  33 --- 94
  33 --- 95
  33 --- 96
  33 --- 97
  33 --- 98
  33 --- 99
  33 --- 100
  33 --- 101
  33 --- 102
  33 --- 103
  34 --- 48
  34 --- 49
  34 --- 50
  34 --- 51
  34 --- 52
  34 --- 53
  34 --- 54
  34 --- 55
  34 --- 56
  34 --- 57
  34 --- 58
  34 --- 59
  34 --- 60
  34 --- 62
  34 --- 64
  34 --- 78
  34 --- 79
  34 --- 80
  34 --- 81
  34 --- 82
  34 --- 83
  34 --- 84
  34 --- 85
  34 --- 86
  34 --- 87
  34 --- 88
  34 --- 89
  34 --- 90
  34 --- 104
  34 --- 105
  34 --- 106
  34 --- 107
  34 --- 108
  34 --- 109
  34 --- 110
  34 --- 111
  34 --- 112
  34 --- 113
  34 --- 114
  34 --- 115
  34 --- 116
  35 --- 74
  99 <--x 35
  35 --- 100
  36 --- 67
  36 --- 92
  100 <--x 36
  37 --- 66
  98 <--x 37
  37 --- 102
  38 --- 69
  96 <--x 38
  38 --- 98
  39 --- 73
  39 --- 91
  97 <--x 39
  40 --- 72
  40 --- 101
  103 <--x 40
  41 --- 76
  91 <--x 41
  41 --- 103
  42 --- 65
  42 --- 96
  101 <--x 42
  43 --- 77
  93 <--x 43
  43 --- 97
  44 --- 75
  44 --- 95
  102 <--x 44
  45 --- 71
  94 <--x 45
  45 --- 99
  46 --- 70
  46 --- 94
  95 <--x 46
  47 --- 68
  92 <--x 47
  47 --- 93
  48 --- 80
  114 <--x 48
  48 --- 116
  49 --- 82
  110 <--x 49
  49 --- 111
  50 --- 78
  50 --- 104
  116 <--x 50
  51 --- 84
  105 <--x 51
  51 --- 106
  52 --- 88
  52 --- 107
  112 <--x 52
  53 --- 87
  53 --- 112
  115 <--x 53
  54 --- 86
  106 <--x 54
  54 --- 109
  55 --- 83
  104 <--x 55
  55 --- 105
  56 --- 90
  113 <--x 56
  56 --- 114
  57 --- 89
  57 --- 108
  109 <--x 57
  58 --- 79
  111 <--x 58
  58 --- 115
  59 --- 85
  107 <--x 59
  59 --- 113
  60 --- 81
  108 <--x 60
  60 --- 110
  65 <--x 63
  66 <--x 63
  67 <--x 63
  68 <--x 63
  69 <--x 63
  70 <--x 63
  71 <--x 63
  72 <--x 63
  73 <--x 63
  74 <--x 63
  75 <--x 63
  76 <--x 63
  77 <--x 63
  78 <--x 64
  79 <--x 64
  80 <--x 64
  81 <--x 64
  82 <--x 64
  83 <--x 64
  84 <--x 64
  85 <--x 64
  86 <--x 64
  87 <--x 64
  88 <--x 64
  89 <--x 64
  90 <--x 64
```
