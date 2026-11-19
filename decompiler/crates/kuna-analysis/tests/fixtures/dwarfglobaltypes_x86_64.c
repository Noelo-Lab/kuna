
#include <stdio.h>
struct config { const char *name; int port; int flags; double scale; };
struct config cfg = { "srv", 8080, 3, 1.5 };
const char *modes[] = { "off", "on", "auto" };
long counter;
int get_port(void) { return cfg.port + cfg.flags; }
const char *mode_name(int i) { counter++; return modes[i]; }
double scaled(int v) { return v * cfg.scale; }
int main(int argc, char **argv) { printf("%s %d %s %f\n", cfg.name, get_port(), mode_name(argc), scaled(argc)); return 0; }
