struct dev { void (*temp)(struct dev *, short *); int pad[16]; short t; };
short readtemp0(struct dev d) { if(d.temp)d.temp(&d,&d.t); return d.t; }
short readtemp1(int a,struct dev d) { if(d.temp)d.temp(&d,&d.t); return d.t+a; }
short readtemp2(int a,int b,struct dev d) { if(d.temp)d.temp(&d,&d.t); return d.t+a+b; }
short readtemp3(int a,int b,int c,struct dev d) { if(d.temp)d.temp(&d,&d.t); return d.t+a+b+c; }
short readtemp4(int a,int b,int c,int e,struct dev d) { if(d.temp)d.temp(&d,&d.t); return d.t+a+b+c+e; }
int readfields(struct dev d) { if(d.temp)d.temp(&d,&d.t); return d.pad[0]+d.pad[15]+d.t; }
struct other { void (*temp)(struct other *, short *); int pad[16]; short t; };
short readother(struct other d) { if(d.temp)d.temp(&d,&d.t); return d.t; }
short readboth(struct dev d,struct other e) { if(d.temp)d.temp(&d,&d.t);if(e.temp)e.temp(&e,&e.t);return d.t+e.t; }
extern void touch_dev(struct dev *);
short untyped(struct dev d) { touch_dev(&d); return d.t; }
