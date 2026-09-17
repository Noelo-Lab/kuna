struct ti { long s, ns; };
long now(void);
void parse(struct ti *, const char *);
void use(struct ti *);
int flag; const char *arg;

void take(void)
{
  struct ti when;
  if (flag)
    parse(&when, arg);
  else {
    long v = now();
    when.s  = v / 1000000000;
    when.ns = v % 1000000000;
  }
  use(&when);
}
