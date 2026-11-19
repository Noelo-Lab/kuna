unsigned observe(unsigned char *p, unsigned n) { return p[0] + n; }

unsigned sample_sp(unsigned n)
{
    unsigned char *p;
    unsigned r, keep;
    keep = _SP;
    _SP -= (n + 1) & ~1;
    p = (unsigned char *)_SP;
    p[0] = 7;
    r = observe(p, n);
    _SP = keep;
    return r + 1;
}

int main(void) { return sample_sp(3); }
