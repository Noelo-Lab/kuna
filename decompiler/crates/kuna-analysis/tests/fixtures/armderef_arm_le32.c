/* armderef fixture: a global reached through a GOT slot (PART=1, -fPIC) and
 * through an absolute literal-pool word (PART=2, -fno-pic), then read and
 * written at constant offsets, before and after a call.
 *
 *   CC=arm-linux-musleabihf-gcc   (GCC 14.2, musl-cross 0.9.11)
 *   $CC -O2 -marm -march=armv7-a -mfpu=vfpv3-d16 -fPIC -DPART=1 -c armderef_arm_le32.c -o p1.o
 *   $CC -O2 -marm -march=armv6 -mfpu=vfp -fno-pic -DPART=2 -c armderef_arm_le32.c -o p2.o
 *   $CC -nostdlib -static -no-pie -Wl,--build-id=none -Wl,-z,max-page-size=0x1000 \
 *       p1.o p2.o -o armderef_arm_le32
 *
 * PART 2 targets ARMv6, which has no movw/movt, so the address is a pool word. */
struct cfg { int a; int b[8]; int *p; };
#if PART == 1
struct cfg g_cfg;
int g_tab[16] = { 1 };
__attribute__((noinline)) void sink(int v) { __asm__ volatile("" :: "r"(v)); }
int pic_read(void) { return g_cfg.b[3] + g_tab[5]; }
void pic_write(int v) { g_cfg.a = v; g_cfg.b[7] = v; }
int pic_keep(void) { struct cfg *c = &g_cfg; sink(c->a); return c->b[2]; }
#else
extern struct cfg g_cfg;
extern void sink(int);
int abs_read(void) { return g_cfg.b[1]; }
void abs_write(int v) { sink(0); g_cfg.b[4] = v; sink(1); g_cfg.a = v; }
int pic_read(void); void pic_write(int); int pic_keep(void);
void _start(void) { pic_write(pic_read() + pic_keep()); abs_write(abs_read()); for (;;) ; }
#endif
