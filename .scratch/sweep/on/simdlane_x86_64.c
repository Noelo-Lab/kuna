// Function: alphabet_init @ 0x401000
void alphabet_init(int a0)
{
  unsigned int v1;
  int v2;
  unsigned int v3;
  int v4;
  
  if (0 <= a0) {
    v1 = a0 + 1;
    if (0xf <= a0) { // branch-flip
      pshufb(ZEXT116(base1),0);
      pshufb(ZEXT116(base2),0);
      v3 = v1 >> 4;
      alpha[0] = base1 + dat_402000;
      alpha[1] = base1 + dat_402001;
      alpha[2] = base1 + dat_402002;
      alpha[3] = base1 + dat_402003;
      alpha[4] = base1 + dat_402004;
      alpha[5] = base1 + dat_402005;
      alpha[6] = base1 + dat_402006;
      alpha[7] = base1 + dat_402007;
      alpha[8] = base1 + dat_402008;
      alpha[9] = base1 + dat_402009;
      alpha[10] = base1 + dat_40200a;
      alpha[0xb] = base1 + dat_40200b;
      alpha[0xc] = base1 + dat_40200c;
      alpha[0xd] = base1 + dat_40200d;
      alpha[0xe] = base1 + dat_40200e;
      alpha[0xf] = base1 + dat_40200f;
      digit[0] = dat_402000 + base2;
      digit[1] = dat_402001 + base2;
      digit[2] = dat_402002 + base2;
      digit[3] = dat_402003 + base2;
      digit[4] = dat_402004 + base2;
      digit[5] = dat_402005 + base2;
      digit[6] = dat_402006 + base2;
      digit[7] = dat_402007 + base2;
      digit[8] = dat_402008 + base2;
      digit[9] = dat_402009 + base2;
      digit[10] = dat_40200a + base2;
      digit[0xb] = dat_40200b + base2;
      digit[0xc] = dat_40200c + base2;
      digit[0xd] = dat_40200d + base2;
      digit[0xe] = dat_40200e + base2;
      digit[0xf] = dat_40200f + base2;
      if (v3 != 1) {
        alpha[0x10] = base1 + dat_402010;
        alpha[0x11] = base1 + dat_402011;
        alpha[0x12] = base1 + dat_402012;
        alpha[0x13] = base1 + dat_402013;
        alpha[0x14] = base1 + dat_402014;
        alpha[0x15] = base1 + dat_402015;
        alpha[0x16] = base1 + dat_402016;
        alpha[0x17] = base1 + dat_402017;
        alpha[0x18] = base1 + dat_402018;
        alpha[0x19] = base1 + dat_402019;
        alpha[0x1a] = base1 + dat_40201a;
        alpha[0x1b] = base1 + dat_40201b;
        alpha[0x1c] = base1 + dat_40201c;
        alpha[0x1d] = base1 + dat_40201d;
        alpha[0x1e] = base1 + dat_40201e;
        alpha[0x1f] = base1 + dat_40201f;
        digit[0x10] = dat_402010 + base2;
        digit[0x11] = dat_402011 + base2;
        digit[0x12] = dat_402012 + base2;
        digit[0x13] = dat_402013 + base2;
        digit[0x14] = dat_402014 + base2;
        digit[0x15] = dat_402015 + base2;
        digit[0x16] = dat_402016 + base2;
        digit[0x17] = dat_402017 + base2;
        digit[0x18] = dat_402018 + base2;
        digit[0x19] = dat_402019 + base2;
        digit[0x1a] = dat_40201a + base2;
        digit[0x1b] = dat_40201b + base2;
        digit[0x1c] = dat_40201c + base2;
        digit[0x1d] = dat_40201d + base2;
        digit[0x1e] = dat_40201e + base2;
        digit[0x1f] = dat_40201f + base2;
        if (v3 != 2) {
          alpha[0x20] = base1 + dat_402020;
          alpha[0x21] = base1 + dat_402021;
          alpha[0x22] = base1 + dat_402022;
          alpha[0x23] = base1 + dat_402023;
          alpha[0x24] = base1 + dat_402024;
          alpha[0x25] = base1 + dat_402025;
          alpha[0x26] = base1 + dat_402026;
          alpha[0x27] = base1 + dat_402027;
          alpha[0x28] = base1 + dat_402028;
          alpha[0x29] = base1 + dat_402029;
          alpha[0x2a] = base1 + dat_40202a;
          alpha[0x2b] = base1 + dat_40202b;
          alpha[0x2c] = base1 + dat_40202c;
          alpha[0x2d] = base1 + dat_40202d;
          alpha[0x2e] = base1 + dat_40202e;
          alpha[0x2f] = base1 + dat_40202f;
          digit[0x20] = dat_402020 + base2;
          digit[0x21] = dat_402021 + base2;
          digit[0x22] = dat_402022 + base2;
          digit[0x23] = dat_402023 + base2;
          digit[0x24] = dat_402024 + base2;
          digit[0x25] = dat_402025 + base2;
          digit[0x26] = dat_402026 + base2;
          digit[0x27] = dat_402027 + base2;
          digit[0x28] = dat_402028 + base2;
          digit[0x29] = dat_402029 + base2;
          digit[0x2a] = dat_40202a + base2;
          digit[0x2b] = dat_40202b + base2;
          digit[0x2c] = dat_40202c + base2;
          digit[0x2d] = dat_40202d + base2;
          digit[0x2e] = dat_40202e + base2;
          digit[0x2f] = dat_40202f + base2;
          if (v3 != 3) {
            alpha[0x30] = base1 + dat_402030;
            alpha[0x31] = base1 + dat_402031;
            alpha[0x32] = base1 + dat_402032;
            alpha[0x33] = base1 + dat_402033;
            alpha[0x34] = base1 + dat_402034;
            alpha[0x35] = base1 + dat_402035;
            alpha[0x36] = base1 + dat_402036;
            alpha[0x37] = base1 + dat_402037;
            alpha[0x38] = base1 + dat_402038;
            alpha[0x39] = base1 + dat_402039;
            alpha[0x3a] = base1 + dat_40203a;
            alpha[0x3b] = base1 + dat_40203b;
            alpha[0x3c] = base1 + dat_40203c;
            alpha[0x3d] = base1 + dat_40203d;
            alpha[0x3e] = base1 + dat_40203e;
            alpha[0x3f] = base1 + dat_40203f;
            digit[0x30] = base2 + dat_402030;
            digit[0x31] = base2 + dat_402031;
            digit[0x32] = base2 + dat_402032;
            digit[0x33] = base2 + dat_402033;
            digit[0x34] = base2 + dat_402034;
            digit[0x35] = base2 + dat_402035;
            digit[0x36] = base2 + dat_402036;
            digit[0x37] = base2 + dat_402037;
            digit[0x38] = base2 + dat_402038;
            digit[0x39] = base2 + dat_402039;
            digit[0x3a] = base2 + dat_40203a;
            digit[0x3b] = base2 + dat_40203b;
            digit[0x3c] = base2 + dat_40203c;
            digit[0x3d] = base2 + dat_40203d;
            digit[0x3e] = base2 + dat_40203e;
            digit[0x3f] = base2 + dat_40203f;
          }
        }
      }
      v3 = v1 & 0xfffffff0;
      if (!(v1 & 0xf)) goto label_40129c;
    }
    else {
      v3 = 0;
    }
    alpha[(int)v3] = base1 + (char)v3;
    digit[(int)v3] = base2 + (char)v3;
    v2 = v3 + 1;
    if (v2 <= a0) {
      digit[v2] = (char)v2 + base2;
      v4 = v3 + 2;
      alpha[v2] = base1 + (char)v2;
      if (v4 <= a0) {
        digit[v4] = (char)v4 + base2;
        v2 = v3 + 3;
        alpha[v4] = base1 + (char)v4;
        if (v2 <= a0) {
          digit[v2] = (char)v2 + base2;
          v4 = v3 + 4;
          alpha[v2] = base1 + (char)v2;
          if (v4 <= a0) {
            digit[v4] = (char)v4 + base2;
            v2 = v3 + 5;
            alpha[v4] = base1 + (char)v4;
            if (v2 <= a0) {
              digit[v2] = (char)v2 + base2;
              v4 = v3 + 6;
              alpha[v2] = base1 + (char)v2;
              if (v4 <= a0) {
                digit[v4] = (char)v4 + base2;
                v2 = v3 + 7;
                alpha[v4] = base1 + (char)v4;
                if (v2 <= a0) {
                  digit[v2] = (char)v2 + base2;
                  v4 = v3 + 8;
                  alpha[v2] = base1 + (char)v2;
                  if (v4 <= a0) {
                    digit[v4] = (char)v4 + base2;
                    v2 = v3 + 9;
                    alpha[v4] = base1 + (char)v4;
                    if (v2 <= a0) {
                      digit[v2] = (char)v2 + base2;
                      v4 = v3 + 10;
                      alpha[v2] = base1 + (char)v2;
                      if (v4 <= a0) {
                        digit[v4] = (char)v4 + base2;
                        v2 = v3 + 0xb;
                        alpha[v4] = base1 + (char)v4;
                        if (v2 <= a0) {
                          digit[v2] = (char)v2 + base2;
                          v4 = v3 + 0xc;
                          alpha[v2] = base1 + (char)v2;
                          if (v4 <= a0) {
                            digit[v4] = (char)v4 + base2;
                            v2 = v3 + 0xd;
                            alpha[v4] = base1 + (char)v4;
                            if (v2 <= a0) {
                              v4 = v3 + 0xe;
                              alpha[v2] = base1 + (char)v2;
                              digit[v2] = (char)v2 + base2;
                              if (v4 <= a0) {
                                alpha[v4] = base1 + (char)v4;
                                digit[v4] = base2 + (char)v4;
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
label_40129c:
  prompt[10] = 0;
  state = 7;
  okmsg[2] = alpha[0x11];
  okmsg[3] = alpha[0x11];
  okmsg[1] = alpha[0xe];
  okmsg[4] = alpha[4];
  okmsg._7_2_ = 0x21;
  prompt._0_8_ = CONCAT71(CONCAT61(CONCAT51(CONCAT41(CONCAT31(CONCAT21(CONCAT11(alpha[3],alpha[0x11]),alpha[0xe]),alpha[0x16]),alpha[0x12]),alpha[0x12]),alpha[0]),alpha[0xf]);
  prompt._8_2_ = 0x203a;
  okmsg[0] = alpha[2];
  okmsg[5] = alpha[2];
  okmsg[6] = alpha[0x13];
  return;
}

// Function: _start @ 0x401380
void _start(void)
{
  alphabet_init(state);
  do {
  } while( true );
}

