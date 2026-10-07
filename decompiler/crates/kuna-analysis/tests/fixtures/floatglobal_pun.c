/* A function that stores a float parameter into gd and also reads gd's bits
   as an integer where the whole-program scan cannot see it. SWITCH: in a case
   of a jump table no range check bounds (k & MASK). Otherwise: through a pointer
   spilled among more stack slots than the scan follows (-O0). The parameter
   keeps its integer type and the integer read prints no conversion of gd.
   x86_64:  gcc -O2 -DSWITCH / clang -O2 -DSWITCH / gcc -O0, each with
            -nostdlib -fno-asynchronous-unwind-tables
   x86_64 switch images are -fPIE -static-pie (a gcc/clang PIE table is
   rebased: lea, movslq, add, jmp). aarch64: aarch64-linux-gnu-gcc -O2
   -DSWITCH -DMASK=15 -fno-pie -c (an object, so the table is a relocation). */
#define NI __attribute__((noinline))
double gd = 2.5;
long g0, g1, g2, g3, g4, g5, g6, g7, g8, g9, g10, g11, g12, g13, g14, g15;
#ifndef MASK
#define MASK 7
#endif
volatile int sinkv;
#ifdef SWITCH
NI void fy(unsigned k, double b) {
  switch (k & MASK) {
  case 0: g0 += 3; break;
  case 1: g1 ^= 5; break;
  case 2: g2 -= 7; break;
  case 3: g3 *= 11; break;
  case 4: g4 += 13; break;
  case 5: g5 |= 17; break;
  case 6: g6 &= 19; break;
  case 7: g7 = *(volatile long *)&gd + 1; break;
#if MASK == 15
  case 8: g8 += 23; break;
  case 9: g9 -= 29; break;
  case 10: g10 ^= 31; break;
  case 11: g11 += 37; break;
  case 12: g12 *= 41; break;
  case 13: g13 |= 43; break;
  case 14: g14 -= 47; break;
  case 15: g15 += 53; break;
#endif
  }
  gd = b;
}
#else
NI void fy(unsigned k, double b) {
  int l0 = 1;
  int l1 = 4;
  int l2 = 7;
  int l3 = 10;
  int l4 = 13;
  int l5 = 16;
  int l6 = 19;
  int l7 = 22;
  int l8 = 25;
  int l9 = 28;
  int l10 = 31;
  int l11 = 34;
  int l12 = 37;
  int l13 = 40;
  int l14 = 43;
  int l15 = 46;
  int l16 = 49;
  int l17 = 52;
  int l18 = 55;
  int l19 = 58;
  int l20 = 61;
  int l21 = 64;
  int l22 = 67;
  int l23 = 70;
  int l24 = 73;
  int l25 = 76;
  int l26 = 79;
  int l27 = 82;
  int l28 = 85;
  int l29 = 88;
  int l30 = 91;
  int l31 = 94;
  int l32 = 97;
  int l33 = 100;
  int l34 = 103;
  int l35 = 106;
  int l36 = 109;
  int l37 = 112;
  int l38 = 115;
  int l39 = 118;
  int l40 = 121;
  int l41 = 124;
  int l42 = 127;
  int l43 = 130;
  int l44 = 133;
  int l45 = 136;
  int l46 = 139;
  int l47 = 142;
  int l48 = 145;
  int l49 = 148;
  int l50 = 151;
  int l51 = 154;
  int l52 = 157;
  int l53 = 160;
  int l54 = 163;
  int l55 = 166;
  int l56 = 169;
  int l57 = 172;
  int l58 = 175;
  int l59 = 178;
  int l60 = 181;
  int l61 = 184;
  int l62 = 187;
  int l63 = 190;
  int l64 = 193;
  int l65 = 196;
  long *q = (long *)&gd;
  g7 = *q + 1;
  sinkv = l0;
  sinkv = l1;
  sinkv = l2;
  sinkv = l3;
  sinkv = l4;
  sinkv = l5;
  sinkv = l6;
  sinkv = l7;
  sinkv = l8;
  sinkv = l9;
  sinkv = l10;
  sinkv = l11;
  sinkv = l12;
  sinkv = l13;
  sinkv = l14;
  sinkv = l15;
  sinkv = l16;
  sinkv = l17;
  sinkv = l18;
  sinkv = l19;
  sinkv = l20;
  sinkv = l21;
  sinkv = l22;
  sinkv = l23;
  sinkv = l24;
  sinkv = l25;
  sinkv = l26;
  sinkv = l27;
  sinkv = l28;
  sinkv = l29;
  sinkv = l30;
  sinkv = l31;
  sinkv = l32;
  sinkv = l33;
  sinkv = l34;
  sinkv = l35;
  sinkv = l36;
  sinkv = l37;
  sinkv = l38;
  sinkv = l39;
  sinkv = l40;
  sinkv = l41;
  sinkv = l42;
  sinkv = l43;
  sinkv = l44;
  sinkv = l45;
  sinkv = l46;
  sinkv = l47;
  sinkv = l48;
  sinkv = l49;
  sinkv = l50;
  sinkv = l51;
  sinkv = l52;
  sinkv = l53;
  sinkv = l54;
  sinkv = l55;
  sinkv = l56;
  sinkv = l57;
  sinkv = l58;
  sinkv = l59;
  sinkv = l60;
  sinkv = l61;
  sinkv = l62;
  sinkv = l63;
  sinkv = l64;
  sinkv = l65;
  gd = b;
}
#endif
NI double rd(void) { return gd * 3.0; }
void _start(void) { fy(sinkv + 7, 4.5); sinkv = (int)rd() + (int)g7 + (int)(g0 + g1 + g2 + g3 + g4 + g5 + g6); for (;;) {} }
