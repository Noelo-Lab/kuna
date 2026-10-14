struct header {
    unsigned int magic;
    unsigned short version;
    unsigned char width;
    unsigned char height;
    unsigned int count;
    unsigned char *pixels;
};

void fetch(void *dst, unsigned long n);
void setup(struct header *h);
int report(unsigned int w, unsigned int h, unsigned long sum);

int parse_header(void)
{
    struct header h = {0};
    fetch(&h, 8);
    if (h.magic != 0x52444845)
        return -1;
    if (h.version != 3)
        return -2;
    setup(&h);
    unsigned long sum = 0;
    for (unsigned int i = 0; i < h.count; i++)
        sum += h.pixels[i];
    return report(h.width, h.height, sum);
}

unsigned long word_sum(void)
{
    unsigned long w = 0;
    fetch(&w, 8);
    return (unsigned int)w + (w >> 32);
}

int text_len(void)
{
    char msg[12] = "hello world";
    fetch(msg, 0);
    return report(msg[0], msg[1], 0);
}
