/* PIC O64 imports through the GOT, a jump table and a global. */
extern void free(void *);
extern unsigned long strlen(const char *);
int counter;
static int table[4] = { 3, 5, 7, 11 };
void release(void *p) { free(p); counter++; }
unsigned long measure(const char *s) { return strlen(s) + 1; }
int pick(int i)
{
    switch (i) {
    case 0: return 10; case 1: return 21; case 2: return 32; case 3: return 43;
    case 4: return 54; case 5: return 65; case 6: return 76; case 7: return 87;
    default: return table[i & 3];
    }
}
int main(void) { return pick(counter); }
