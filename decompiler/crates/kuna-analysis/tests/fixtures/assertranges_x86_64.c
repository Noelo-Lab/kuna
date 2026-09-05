/* Fixture for the `--assert` range directives — `readonly`, `volatile`, and the
 * `global add`/`global remove` console pair (kuna RE-need
 * `no-cli-data-code-override`).
 *
 * Three facts an agent knows and kuna cannot derive:
 *
 *   - `scale` and `bias` live in `.data`, which is writable, so the loader never
 *     reports them read-only -- but nothing ever stores to them, so their
 *     initialisers are the values the program runs with. Asserting the pair
 *     read-only folds the two loads in `sample` into `* 7` and `+ 100`.
 *   - 0x50000000 is device memory outside every loaded section. `sample`'s two
 *     reads are distinct hardware accesses, but to the decompiler they are two
 *     loads of one unwritten address and CSE merges them. Asserting the range
 *     volatile keeps both.
 *   - `latch`'s store to 0x50000004 survives the call only because the address
 *     is inside the global scope's range (every stock cspec claims the whole
 *     `ram` space); `global remove` takes it out and the store dies, `global
 *     add` puts it back.
 *
 * Build (fixed VMAs, so a probe can name the addresses):
 *   gcc -O2 -no-pie -fno-stack-protector -fcf-protection=none \
 *       -o assertranges_x86_64 assertranges_x86_64.c
 */

int scale = 7;
int bias = 100;

__attribute__((noinline, noipa)) void ext(void)
{
}

__attribute__((noinline)) int sample(int n)
{
	int a = *(volatile int *)0x50000000;
	int b = *(volatile int *)0x50000000;
	return n * scale + bias + a + b;
}

__attribute__((noinline)) int latch(int n)
{
	*(int *)0x50000004 = n;
	ext();
	return *(int *)0x50000004;
}

int main(void)
{
	return sample(3) + latch(1);
}
