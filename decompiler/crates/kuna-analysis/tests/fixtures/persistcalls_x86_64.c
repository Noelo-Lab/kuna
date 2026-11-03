/* GH-1034 `persistcalls` fixture. `churn` reads, then writes, each of 256
   writable globals in .data, with a call to `sink` after every write and one
   more inside a branch every eighth step: 288 calls, so a heritage pass guards
   256 persistent ranges at 288 calls (73,728 guards, past persistcalls'
   65,536-guard threshold). `sink` clobbers memory, so every global is read
   again after the calls before it. `small` touches two globals across three
   calls, far below the threshold.
   Rebuild:
     gcc -O2 -fno-pie -no-pie -fno-inline -o persistcalls_x86_64 persistcalls_x86_64.c
   Run with no arguments it prints 23817576453 67 23817685162 76. */
#include <stdio.h>
long g0 = 3, g1 = 10, g2 = 17, g3 = 24, g4 = 31, g5 = 38, g6 = 45, g7 = 52, g8 = 59, g9 = 66, g10 = 73, g11 = 80, g12 = 87, g13 = 94, g14 = 101, g15 = 108, g16 = 115, g17 = 122, g18 = 129, g19 = 136, g20 = 143, g21 = 150, g22 = 157, g23 = 164, g24 = 171, g25 = 178, g26 = 185, g27 = 192, g28 = 199, g29 = 206, g30 = 213, g31 = 220, g32 = 227, g33 = 234, g34 = 241, g35 = 248, g36 = 255, g37 = 262, g38 = 269, g39 = 276, g40 = 283, g41 = 290, g42 = 297, g43 = 304, g44 = 311, g45 = 318, g46 = 325, g47 = 332, g48 = 339, g49 = 346, g50 = 353, g51 = 360, g52 = 367, g53 = 374, g54 = 381, g55 = 388, g56 = 395, g57 = 402, g58 = 409, g59 = 416, g60 = 423, g61 = 430, g62 = 437, g63 = 444, g64 = 451, g65 = 458, g66 = 465, g67 = 472, g68 = 479, g69 = 486, g70 = 493, g71 = 500, g72 = 507, g73 = 514, g74 = 521, g75 = 528, g76 = 535, g77 = 542, g78 = 549, g79 = 556, g80 = 563, g81 = 570, g82 = 577, g83 = 584, g84 = 591, g85 = 598, g86 = 605, g87 = 612, g88 = 619, g89 = 626, g90 = 633, g91 = 640, g92 = 647, g93 = 654, g94 = 661, g95 = 668, g96 = 675, g97 = 682, g98 = 689, g99 = 696, g100 = 703, g101 = 710, g102 = 717, g103 = 724, g104 = 731, g105 = 738, g106 = 745, g107 = 752, g108 = 759, g109 = 766, g110 = 773, g111 = 780, g112 = 787, g113 = 794, g114 = 801, g115 = 808, g116 = 815, g117 = 822, g118 = 829, g119 = 836, g120 = 843, g121 = 850, g122 = 857, g123 = 864, g124 = 871, g125 = 878, g126 = 885, g127 = 892, g128 = 899, g129 = 906, g130 = 913, g131 = 920, g132 = 927, g133 = 934, g134 = 941, g135 = 948, g136 = 955, g137 = 962, g138 = 969, g139 = 976, g140 = 983, g141 = 990, g142 = 997, g143 = 1004, g144 = 1011, g145 = 1018, g146 = 1025, g147 = 1032, g148 = 1039, g149 = 1046, g150 = 1053, g151 = 1060, g152 = 1067, g153 = 1074, g154 = 1081, g155 = 1088, g156 = 1095, g157 = 1102, g158 = 1109, g159 = 1116, g160 = 1123, g161 = 1130, g162 = 1137, g163 = 1144, g164 = 1151, g165 = 1158, g166 = 1165, g167 = 1172, g168 = 1179, g169 = 1186, g170 = 1193, g171 = 1200, g172 = 1207, g173 = 1214, g174 = 1221, g175 = 1228, g176 = 1235, g177 = 1242, g178 = 1249, g179 = 1256, g180 = 1263, g181 = 1270, g182 = 1277, g183 = 1284, g184 = 1291, g185 = 1298, g186 = 1305, g187 = 1312, g188 = 1319, g189 = 1326, g190 = 1333, g191 = 1340, g192 = 1347, g193 = 1354, g194 = 1361, g195 = 1368, g196 = 1375, g197 = 1382, g198 = 1389, g199 = 1396, g200 = 1403, g201 = 1410, g202 = 1417, g203 = 1424, g204 = 1431, g205 = 1438, g206 = 1445, g207 = 1452, g208 = 1459, g209 = 1466, g210 = 1473, g211 = 1480, g212 = 1487, g213 = 1494, g214 = 1501, g215 = 1508, g216 = 1515, g217 = 1522, g218 = 1529, g219 = 1536, g220 = 1543, g221 = 1550, g222 = 1557, g223 = 1564, g224 = 1571, g225 = 1578, g226 = 1585, g227 = 1592, g228 = 1599, g229 = 1606, g230 = 1613, g231 = 1620, g232 = 1627, g233 = 1634, g234 = 1641, g235 = 1648, g236 = 1655, g237 = 1662, g238 = 1669, g239 = 1676, g240 = 1683, g241 = 1690, g242 = 1697, g243 = 1704, g244 = 1711, g245 = 1718, g246 = 1725, g247 = 1732, g248 = 1739, g249 = 1746, g250 = 1753, g251 = 1760, g252 = 1767, g253 = 1774, g254 = 1781, g255 = 1788;
long h0 = 11, h1 = 13;
__attribute__((noinline)) long sink(long v) { __asm__ volatile("" : "+r"(v) : : "memory"); return v * 3 + 1; }
__attribute__((noinline)) long churn(long x) {
  long acc = x;
  { long t = g0; g0 = t * 5 + acc; acc ^= sink(t); }
  { long t = g1; g1 = t * 5 + acc; acc ^= sink(t); }
  { long t = g2; g2 = t * 5 + acc; acc ^= sink(t); }
  { long t = g3; g3 = t * 5 + acc; acc ^= sink(t); }
  { long t = g4; g4 = t * 5 + acc; acc ^= sink(t); }
  { long t = g5; g5 = t * 5 + acc; acc ^= sink(t); }
  { long t = g6; g6 = t * 5 + acc; acc ^= sink(t); }
  { long t = g7; g7 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 1) acc += sink(acc);
  { long t = g8; g8 = t * 5 + acc; acc ^= sink(t); }
  { long t = g9; g9 = t * 5 + acc; acc ^= sink(t); }
  { long t = g10; g10 = t * 5 + acc; acc ^= sink(t); }
  { long t = g11; g11 = t * 5 + acc; acc ^= sink(t); }
  { long t = g12; g12 = t * 5 + acc; acc ^= sink(t); }
  { long t = g13; g13 = t * 5 + acc; acc ^= sink(t); }
  { long t = g14; g14 = t * 5 + acc; acc ^= sink(t); }
  { long t = g15; g15 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 2) acc += sink(acc);
  { long t = g16; g16 = t * 5 + acc; acc ^= sink(t); }
  { long t = g17; g17 = t * 5 + acc; acc ^= sink(t); }
  { long t = g18; g18 = t * 5 + acc; acc ^= sink(t); }
  { long t = g19; g19 = t * 5 + acc; acc ^= sink(t); }
  { long t = g20; g20 = t * 5 + acc; acc ^= sink(t); }
  { long t = g21; g21 = t * 5 + acc; acc ^= sink(t); }
  { long t = g22; g22 = t * 5 + acc; acc ^= sink(t); }
  { long t = g23; g23 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 4) acc += sink(acc);
  { long t = g24; g24 = t * 5 + acc; acc ^= sink(t); }
  { long t = g25; g25 = t * 5 + acc; acc ^= sink(t); }
  { long t = g26; g26 = t * 5 + acc; acc ^= sink(t); }
  { long t = g27; g27 = t * 5 + acc; acc ^= sink(t); }
  { long t = g28; g28 = t * 5 + acc; acc ^= sink(t); }
  { long t = g29; g29 = t * 5 + acc; acc ^= sink(t); }
  { long t = g30; g30 = t * 5 + acc; acc ^= sink(t); }
  { long t = g31; g31 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 8) acc += sink(acc);
  { long t = g32; g32 = t * 5 + acc; acc ^= sink(t); }
  { long t = g33; g33 = t * 5 + acc; acc ^= sink(t); }
  { long t = g34; g34 = t * 5 + acc; acc ^= sink(t); }
  { long t = g35; g35 = t * 5 + acc; acc ^= sink(t); }
  { long t = g36; g36 = t * 5 + acc; acc ^= sink(t); }
  { long t = g37; g37 = t * 5 + acc; acc ^= sink(t); }
  { long t = g38; g38 = t * 5 + acc; acc ^= sink(t); }
  { long t = g39; g39 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 16) acc += sink(acc);
  { long t = g40; g40 = t * 5 + acc; acc ^= sink(t); }
  { long t = g41; g41 = t * 5 + acc; acc ^= sink(t); }
  { long t = g42; g42 = t * 5 + acc; acc ^= sink(t); }
  { long t = g43; g43 = t * 5 + acc; acc ^= sink(t); }
  { long t = g44; g44 = t * 5 + acc; acc ^= sink(t); }
  { long t = g45; g45 = t * 5 + acc; acc ^= sink(t); }
  { long t = g46; g46 = t * 5 + acc; acc ^= sink(t); }
  { long t = g47; g47 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 32) acc += sink(acc);
  { long t = g48; g48 = t * 5 + acc; acc ^= sink(t); }
  { long t = g49; g49 = t * 5 + acc; acc ^= sink(t); }
  { long t = g50; g50 = t * 5 + acc; acc ^= sink(t); }
  { long t = g51; g51 = t * 5 + acc; acc ^= sink(t); }
  { long t = g52; g52 = t * 5 + acc; acc ^= sink(t); }
  { long t = g53; g53 = t * 5 + acc; acc ^= sink(t); }
  { long t = g54; g54 = t * 5 + acc; acc ^= sink(t); }
  { long t = g55; g55 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 64) acc += sink(acc);
  { long t = g56; g56 = t * 5 + acc; acc ^= sink(t); }
  { long t = g57; g57 = t * 5 + acc; acc ^= sink(t); }
  { long t = g58; g58 = t * 5 + acc; acc ^= sink(t); }
  { long t = g59; g59 = t * 5 + acc; acc ^= sink(t); }
  { long t = g60; g60 = t * 5 + acc; acc ^= sink(t); }
  { long t = g61; g61 = t * 5 + acc; acc ^= sink(t); }
  { long t = g62; g62 = t * 5 + acc; acc ^= sink(t); }
  { long t = g63; g63 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 128) acc += sink(acc);
  { long t = g64; g64 = t * 5 + acc; acc ^= sink(t); }
  { long t = g65; g65 = t * 5 + acc; acc ^= sink(t); }
  { long t = g66; g66 = t * 5 + acc; acc ^= sink(t); }
  { long t = g67; g67 = t * 5 + acc; acc ^= sink(t); }
  { long t = g68; g68 = t * 5 + acc; acc ^= sink(t); }
  { long t = g69; g69 = t * 5 + acc; acc ^= sink(t); }
  { long t = g70; g70 = t * 5 + acc; acc ^= sink(t); }
  { long t = g71; g71 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 256) acc += sink(acc);
  { long t = g72; g72 = t * 5 + acc; acc ^= sink(t); }
  { long t = g73; g73 = t * 5 + acc; acc ^= sink(t); }
  { long t = g74; g74 = t * 5 + acc; acc ^= sink(t); }
  { long t = g75; g75 = t * 5 + acc; acc ^= sink(t); }
  { long t = g76; g76 = t * 5 + acc; acc ^= sink(t); }
  { long t = g77; g77 = t * 5 + acc; acc ^= sink(t); }
  { long t = g78; g78 = t * 5 + acc; acc ^= sink(t); }
  { long t = g79; g79 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 512) acc += sink(acc);
  { long t = g80; g80 = t * 5 + acc; acc ^= sink(t); }
  { long t = g81; g81 = t * 5 + acc; acc ^= sink(t); }
  { long t = g82; g82 = t * 5 + acc; acc ^= sink(t); }
  { long t = g83; g83 = t * 5 + acc; acc ^= sink(t); }
  { long t = g84; g84 = t * 5 + acc; acc ^= sink(t); }
  { long t = g85; g85 = t * 5 + acc; acc ^= sink(t); }
  { long t = g86; g86 = t * 5 + acc; acc ^= sink(t); }
  { long t = g87; g87 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 1024) acc += sink(acc);
  { long t = g88; g88 = t * 5 + acc; acc ^= sink(t); }
  { long t = g89; g89 = t * 5 + acc; acc ^= sink(t); }
  { long t = g90; g90 = t * 5 + acc; acc ^= sink(t); }
  { long t = g91; g91 = t * 5 + acc; acc ^= sink(t); }
  { long t = g92; g92 = t * 5 + acc; acc ^= sink(t); }
  { long t = g93; g93 = t * 5 + acc; acc ^= sink(t); }
  { long t = g94; g94 = t * 5 + acc; acc ^= sink(t); }
  { long t = g95; g95 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 2048) acc += sink(acc);
  { long t = g96; g96 = t * 5 + acc; acc ^= sink(t); }
  { long t = g97; g97 = t * 5 + acc; acc ^= sink(t); }
  { long t = g98; g98 = t * 5 + acc; acc ^= sink(t); }
  { long t = g99; g99 = t * 5 + acc; acc ^= sink(t); }
  { long t = g100; g100 = t * 5 + acc; acc ^= sink(t); }
  { long t = g101; g101 = t * 5 + acc; acc ^= sink(t); }
  { long t = g102; g102 = t * 5 + acc; acc ^= sink(t); }
  { long t = g103; g103 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 4096) acc += sink(acc);
  { long t = g104; g104 = t * 5 + acc; acc ^= sink(t); }
  { long t = g105; g105 = t * 5 + acc; acc ^= sink(t); }
  { long t = g106; g106 = t * 5 + acc; acc ^= sink(t); }
  { long t = g107; g107 = t * 5 + acc; acc ^= sink(t); }
  { long t = g108; g108 = t * 5 + acc; acc ^= sink(t); }
  { long t = g109; g109 = t * 5 + acc; acc ^= sink(t); }
  { long t = g110; g110 = t * 5 + acc; acc ^= sink(t); }
  { long t = g111; g111 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 8192) acc += sink(acc);
  { long t = g112; g112 = t * 5 + acc; acc ^= sink(t); }
  { long t = g113; g113 = t * 5 + acc; acc ^= sink(t); }
  { long t = g114; g114 = t * 5 + acc; acc ^= sink(t); }
  { long t = g115; g115 = t * 5 + acc; acc ^= sink(t); }
  { long t = g116; g116 = t * 5 + acc; acc ^= sink(t); }
  { long t = g117; g117 = t * 5 + acc; acc ^= sink(t); }
  { long t = g118; g118 = t * 5 + acc; acc ^= sink(t); }
  { long t = g119; g119 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 16384) acc += sink(acc);
  { long t = g120; g120 = t * 5 + acc; acc ^= sink(t); }
  { long t = g121; g121 = t * 5 + acc; acc ^= sink(t); }
  { long t = g122; g122 = t * 5 + acc; acc ^= sink(t); }
  { long t = g123; g123 = t * 5 + acc; acc ^= sink(t); }
  { long t = g124; g124 = t * 5 + acc; acc ^= sink(t); }
  { long t = g125; g125 = t * 5 + acc; acc ^= sink(t); }
  { long t = g126; g126 = t * 5 + acc; acc ^= sink(t); }
  { long t = g127; g127 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 32768) acc += sink(acc);
  { long t = g128; g128 = t * 5 + acc; acc ^= sink(t); }
  { long t = g129; g129 = t * 5 + acc; acc ^= sink(t); }
  { long t = g130; g130 = t * 5 + acc; acc ^= sink(t); }
  { long t = g131; g131 = t * 5 + acc; acc ^= sink(t); }
  { long t = g132; g132 = t * 5 + acc; acc ^= sink(t); }
  { long t = g133; g133 = t * 5 + acc; acc ^= sink(t); }
  { long t = g134; g134 = t * 5 + acc; acc ^= sink(t); }
  { long t = g135; g135 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 1) acc += sink(acc);
  { long t = g136; g136 = t * 5 + acc; acc ^= sink(t); }
  { long t = g137; g137 = t * 5 + acc; acc ^= sink(t); }
  { long t = g138; g138 = t * 5 + acc; acc ^= sink(t); }
  { long t = g139; g139 = t * 5 + acc; acc ^= sink(t); }
  { long t = g140; g140 = t * 5 + acc; acc ^= sink(t); }
  { long t = g141; g141 = t * 5 + acc; acc ^= sink(t); }
  { long t = g142; g142 = t * 5 + acc; acc ^= sink(t); }
  { long t = g143; g143 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 2) acc += sink(acc);
  { long t = g144; g144 = t * 5 + acc; acc ^= sink(t); }
  { long t = g145; g145 = t * 5 + acc; acc ^= sink(t); }
  { long t = g146; g146 = t * 5 + acc; acc ^= sink(t); }
  { long t = g147; g147 = t * 5 + acc; acc ^= sink(t); }
  { long t = g148; g148 = t * 5 + acc; acc ^= sink(t); }
  { long t = g149; g149 = t * 5 + acc; acc ^= sink(t); }
  { long t = g150; g150 = t * 5 + acc; acc ^= sink(t); }
  { long t = g151; g151 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 4) acc += sink(acc);
  { long t = g152; g152 = t * 5 + acc; acc ^= sink(t); }
  { long t = g153; g153 = t * 5 + acc; acc ^= sink(t); }
  { long t = g154; g154 = t * 5 + acc; acc ^= sink(t); }
  { long t = g155; g155 = t * 5 + acc; acc ^= sink(t); }
  { long t = g156; g156 = t * 5 + acc; acc ^= sink(t); }
  { long t = g157; g157 = t * 5 + acc; acc ^= sink(t); }
  { long t = g158; g158 = t * 5 + acc; acc ^= sink(t); }
  { long t = g159; g159 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 8) acc += sink(acc);
  { long t = g160; g160 = t * 5 + acc; acc ^= sink(t); }
  { long t = g161; g161 = t * 5 + acc; acc ^= sink(t); }
  { long t = g162; g162 = t * 5 + acc; acc ^= sink(t); }
  { long t = g163; g163 = t * 5 + acc; acc ^= sink(t); }
  { long t = g164; g164 = t * 5 + acc; acc ^= sink(t); }
  { long t = g165; g165 = t * 5 + acc; acc ^= sink(t); }
  { long t = g166; g166 = t * 5 + acc; acc ^= sink(t); }
  { long t = g167; g167 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 16) acc += sink(acc);
  { long t = g168; g168 = t * 5 + acc; acc ^= sink(t); }
  { long t = g169; g169 = t * 5 + acc; acc ^= sink(t); }
  { long t = g170; g170 = t * 5 + acc; acc ^= sink(t); }
  { long t = g171; g171 = t * 5 + acc; acc ^= sink(t); }
  { long t = g172; g172 = t * 5 + acc; acc ^= sink(t); }
  { long t = g173; g173 = t * 5 + acc; acc ^= sink(t); }
  { long t = g174; g174 = t * 5 + acc; acc ^= sink(t); }
  { long t = g175; g175 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 32) acc += sink(acc);
  { long t = g176; g176 = t * 5 + acc; acc ^= sink(t); }
  { long t = g177; g177 = t * 5 + acc; acc ^= sink(t); }
  { long t = g178; g178 = t * 5 + acc; acc ^= sink(t); }
  { long t = g179; g179 = t * 5 + acc; acc ^= sink(t); }
  { long t = g180; g180 = t * 5 + acc; acc ^= sink(t); }
  { long t = g181; g181 = t * 5 + acc; acc ^= sink(t); }
  { long t = g182; g182 = t * 5 + acc; acc ^= sink(t); }
  { long t = g183; g183 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 64) acc += sink(acc);
  { long t = g184; g184 = t * 5 + acc; acc ^= sink(t); }
  { long t = g185; g185 = t * 5 + acc; acc ^= sink(t); }
  { long t = g186; g186 = t * 5 + acc; acc ^= sink(t); }
  { long t = g187; g187 = t * 5 + acc; acc ^= sink(t); }
  { long t = g188; g188 = t * 5 + acc; acc ^= sink(t); }
  { long t = g189; g189 = t * 5 + acc; acc ^= sink(t); }
  { long t = g190; g190 = t * 5 + acc; acc ^= sink(t); }
  { long t = g191; g191 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 128) acc += sink(acc);
  { long t = g192; g192 = t * 5 + acc; acc ^= sink(t); }
  { long t = g193; g193 = t * 5 + acc; acc ^= sink(t); }
  { long t = g194; g194 = t * 5 + acc; acc ^= sink(t); }
  { long t = g195; g195 = t * 5 + acc; acc ^= sink(t); }
  { long t = g196; g196 = t * 5 + acc; acc ^= sink(t); }
  { long t = g197; g197 = t * 5 + acc; acc ^= sink(t); }
  { long t = g198; g198 = t * 5 + acc; acc ^= sink(t); }
  { long t = g199; g199 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 256) acc += sink(acc);
  { long t = g200; g200 = t * 5 + acc; acc ^= sink(t); }
  { long t = g201; g201 = t * 5 + acc; acc ^= sink(t); }
  { long t = g202; g202 = t * 5 + acc; acc ^= sink(t); }
  { long t = g203; g203 = t * 5 + acc; acc ^= sink(t); }
  { long t = g204; g204 = t * 5 + acc; acc ^= sink(t); }
  { long t = g205; g205 = t * 5 + acc; acc ^= sink(t); }
  { long t = g206; g206 = t * 5 + acc; acc ^= sink(t); }
  { long t = g207; g207 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 512) acc += sink(acc);
  { long t = g208; g208 = t * 5 + acc; acc ^= sink(t); }
  { long t = g209; g209 = t * 5 + acc; acc ^= sink(t); }
  { long t = g210; g210 = t * 5 + acc; acc ^= sink(t); }
  { long t = g211; g211 = t * 5 + acc; acc ^= sink(t); }
  { long t = g212; g212 = t * 5 + acc; acc ^= sink(t); }
  { long t = g213; g213 = t * 5 + acc; acc ^= sink(t); }
  { long t = g214; g214 = t * 5 + acc; acc ^= sink(t); }
  { long t = g215; g215 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 1024) acc += sink(acc);
  { long t = g216; g216 = t * 5 + acc; acc ^= sink(t); }
  { long t = g217; g217 = t * 5 + acc; acc ^= sink(t); }
  { long t = g218; g218 = t * 5 + acc; acc ^= sink(t); }
  { long t = g219; g219 = t * 5 + acc; acc ^= sink(t); }
  { long t = g220; g220 = t * 5 + acc; acc ^= sink(t); }
  { long t = g221; g221 = t * 5 + acc; acc ^= sink(t); }
  { long t = g222; g222 = t * 5 + acc; acc ^= sink(t); }
  { long t = g223; g223 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 2048) acc += sink(acc);
  { long t = g224; g224 = t * 5 + acc; acc ^= sink(t); }
  { long t = g225; g225 = t * 5 + acc; acc ^= sink(t); }
  { long t = g226; g226 = t * 5 + acc; acc ^= sink(t); }
  { long t = g227; g227 = t * 5 + acc; acc ^= sink(t); }
  { long t = g228; g228 = t * 5 + acc; acc ^= sink(t); }
  { long t = g229; g229 = t * 5 + acc; acc ^= sink(t); }
  { long t = g230; g230 = t * 5 + acc; acc ^= sink(t); }
  { long t = g231; g231 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 4096) acc += sink(acc);
  { long t = g232; g232 = t * 5 + acc; acc ^= sink(t); }
  { long t = g233; g233 = t * 5 + acc; acc ^= sink(t); }
  { long t = g234; g234 = t * 5 + acc; acc ^= sink(t); }
  { long t = g235; g235 = t * 5 + acc; acc ^= sink(t); }
  { long t = g236; g236 = t * 5 + acc; acc ^= sink(t); }
  { long t = g237; g237 = t * 5 + acc; acc ^= sink(t); }
  { long t = g238; g238 = t * 5 + acc; acc ^= sink(t); }
  { long t = g239; g239 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 8192) acc += sink(acc);
  { long t = g240; g240 = t * 5 + acc; acc ^= sink(t); }
  { long t = g241; g241 = t * 5 + acc; acc ^= sink(t); }
  { long t = g242; g242 = t * 5 + acc; acc ^= sink(t); }
  { long t = g243; g243 = t * 5 + acc; acc ^= sink(t); }
  { long t = g244; g244 = t * 5 + acc; acc ^= sink(t); }
  { long t = g245; g245 = t * 5 + acc; acc ^= sink(t); }
  { long t = g246; g246 = t * 5 + acc; acc ^= sink(t); }
  { long t = g247; g247 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 16384) acc += sink(acc);
  { long t = g248; g248 = t * 5 + acc; acc ^= sink(t); }
  { long t = g249; g249 = t * 5 + acc; acc ^= sink(t); }
  { long t = g250; g250 = t * 5 + acc; acc ^= sink(t); }
  { long t = g251; g251 = t * 5 + acc; acc ^= sink(t); }
  { long t = g252; g252 = t * 5 + acc; acc ^= sink(t); }
  { long t = g253; g253 = t * 5 + acc; acc ^= sink(t); }
  { long t = g254; g254 = t * 5 + acc; acc ^= sink(t); }
  { long t = g255; g255 = t * 5 + acc; acc ^= sink(t); }
  if (acc & 32768) acc += sink(acc);
  return acc;
}
__attribute__((noinline)) long small(long x) {
  long a = sink(x);
  h0 = h0 + a;
  a = sink(a);
  long b = h1;
  h1 = a;
  return sink(b) + h0;
}
int main(void) {
  long r = churn(7);
  long s = small(r & 0xff);
  printf("%ld %ld %ld %ld\n", r, s, g0 + g100 + g255, h0 + h1);
  return 0;
}
