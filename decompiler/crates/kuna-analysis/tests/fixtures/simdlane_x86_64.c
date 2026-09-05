char base1 = 97, base2 = 48;
char alpha[64]; char digit[64]; char prompt[32]; char okmsg[32]; int state;
void alphabet_init(int n)
{
  int i;
  if (n >= 0) {
    for (i = 0; i <= n; i++) { alpha[i] = (char)(base1 + i); digit[i] = (char)(base2 + i); }
    if (n == 63) state = 2;
  }
  prompt[0]=alpha[15]; prompt[1]=alpha[0]; prompt[2]=alpha[18]; prompt[3]=alpha[18];
  prompt[4]=alpha[22]; prompt[5]=alpha[14]; prompt[6]=alpha[17]; prompt[7]=alpha[3];
  prompt[8]=':'; prompt[9]=' '; prompt[10]=0;
  okmsg[0]=alpha[2]; okmsg[1]=alpha[14]; okmsg[2]=alpha[17]; okmsg[3]=alpha[17];
  okmsg[4]=alpha[4]; okmsg[5]=alpha[2]; okmsg[6]=alpha[19]; okmsg[7]='!'; okmsg[8]=0;
  state = 7;
}
void _start(void){ alphabet_init(state); for(;;); }
