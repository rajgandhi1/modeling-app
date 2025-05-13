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
  5 --- 67
  5 --- 103
  6 --- 40
  6 x--> 61
  6 --- 72
  6 --- 96
  7 --- 42
  7 x--> 61
  7 --- 74
  7 --- 98
  8 --- 38
  8 x--> 61
  8 --- 75
  8 --- 94
  9 --- 37
  9 x--> 61
  9 --- 66
  9 --- 99
  10 --- 44
  10 x--> 61
  10 --- 65
  10 --- 91
  11 --- 46
  11 x--> 61
  11 --- 73
  11 --- 101
  12 --- 45
  12 x--> 61
  12 --- 68
  12 --- 93
  13 --- 35
  13 x--> 61
  13 --- 69
  13 --- 92
  14 --- 36
  14 x--> 61
  14 --- 71
  14 --- 100
  15 --- 47
  15 x--> 61
  15 --- 76
  15 --- 102
  16 --- 43
  16 x--> 61
  16 --- 77
  16 --- 97
  17 --- 39
  17 x--> 61
  17 --- 70
  17 --- 95
  18 --- 51
  18 x--> 62
  18 --- 80
  18 --- 108
  19 --- 54
  19 x--> 62
  19 --- 90
  19 --- 115
  20 --- 57
  20 x--> 62
  20 --- 83
  20 --- 107
  21 --- 60
  21 x--> 62
  21 --- 81
  21 --- 113
  22 --- 49
  22 x--> 62
  22 --- 84
  22 --- 116
  23 --- 58
  23 x--> 62
  23 --- 85
  23 --- 110
  24 --- 53
  24 x--> 62
  24 --- 88
  24 --- 104
  25 --- 52
  25 x--> 62
  25 --- 82
  25 --- 112
  26 --- 59
  26 x--> 62
  26 --- 86
  26 --- 106
  27 --- 56
  27 x--> 62
  27 --- 87
  27 --- 105
  28 --- 48
  28 x--> 62
  28 --- 79
  28 --- 114
  29 --- 50
  29 x--> 62
  29 --- 89
  29 --- 109
  30 --- 55
  30 x--> 62
  30 --- 78
  30 --- 111
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
  35 --- 69
  35 --- 92
  93 <--x 35
  36 --- 71
  92 <--x 36
  36 --- 100
  37 --- 66
  94 <--x 37
  37 --- 99
  38 --- 75
  38 --- 94
  98 <--x 38
  39 --- 70
  39 --- 95
  97 <--x 39
  40 --- 72
  40 --- 96
  103 <--x 40
  41 --- 67
  95 <--x 41
  41 --- 103
  42 --- 74
  96 <--x 42
  42 --- 98
  43 --- 77
  43 --- 97
  102 <--x 43
  44 --- 65
  44 --- 91
  99 <--x 44
  45 --- 68
  45 --- 93
  101 <--x 45
  46 --- 73
  91 <--x 46
  46 --- 101
  47 --- 76
  100 <--x 47
  47 --- 102
  48 --- 79
  105 <--x 48
  48 --- 114
  49 --- 84
  113 <--x 49
  49 --- 116
  50 --- 89
  50 --- 109
  114 <--x 50
  51 --- 80
  51 --- 108
  111 <--x 51
  52 --- 82
  104 <--x 52
  52 --- 112
  53 --- 88
  53 --- 104
  110 <--x 53
  54 --- 90
  108 <--x 54
  54 --- 115
  55 --- 78
  109 <--x 55
  55 --- 111
  56 --- 87
  56 --- 105
  106 <--x 56
  57 --- 83
  57 --- 107
  115 <--x 57
  58 --- 85
  58 --- 110
  116 <--x 58
  59 --- 86
  59 --- 106
  112 <--x 59
  60 --- 81
  107 <--x 60
  60 --- 113
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
