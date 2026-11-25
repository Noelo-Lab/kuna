struct { int type; int *key; long *value; int max_entries; } counters __attribute__((section(".maps")));
static void *(*bpf_map_lookup_elem)(void *map, const void *key) = (void *)1;
__attribute__((section("xdp"))) int prog(void *ctx)
{ int key = 0; long *v = bpf_map_lookup_elem(&counters, &key); if (v) *v += 1; return 2; }
