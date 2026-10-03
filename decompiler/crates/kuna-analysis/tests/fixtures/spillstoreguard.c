int array_write(int a,int x,int y) { int s[2]={x,y}; int *p=s; p[1]=a*3; return s[1]; }
int array_neighbors(int a,int x,int y) { int s[2]={x,y}; int *p=s; p[1]=a*3; return s[0]+s[1]; }
int shifted_write(int a,int x,int y) { int s[2]={x,y}; int *p=s+1; *p=a*3; return s[1]; }
int array_snapshot(int a,int x,int y) { int s[2]={x,y}; int *p=s; int old=s[1]; p[1]=a*3; return old+s[1]*5; }
int array_nonalias(int a,int x,int y) { int s[2]={x,y}; int *p=s; p[0]=a*3; return s[1]; }
int pointer_move(int a,int x,int y) { int s[2]={x,y}; int *p=s; p=s+1; *p=a*3; return s[1]; }
int array_two_writes(int a,int x,int y) { int s[2]={x,y}; int *p=s; p[1]=a*3; p[0]=a*7; return s[0]+s[1]; }
void array_export(int a,int x,int y,int *out) { int s[2]={x,y}; int *p=s; p[1]=a*3; out[0]=s[0]; out[1]=s[1]; }
extern void move_pointer(int **p,int *q);
int array_escape(int a,int x,int y) { int s[2]={x,y}; int *p=s; move_pointer(&p,s+1); *p=a*3; return s[0]+s[1]; }
int array_choose_nonalias(int a,int x,int y) { int s[3]={x,y,17}; int *p=a?s:s+1; *p=a*3; return s[2]; }
int array_constant_write(int a,int x,int y) { int s[4]={x,y,17,23}; int *p=s; p+=2; *p=a*3; return s[0]+s[1]*3+s[2]*5+s[3]*7; }
int array_constant_nonalias(int a,int x,int y) { int s[4]={x,y,17,23}; int *p=s; p+=2; *p=a*3; return s[3]; }
