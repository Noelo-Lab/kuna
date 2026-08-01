extern void fill7(int a,int b,int c,int d,int e,int f,int *out);
int work2(char *buf, int n) {
    int len = 0, i = 0, total = 0;
    for (;;) {
        if (i >= len) { fill7(1,2,3,4,5,6,&len); if (len == 0) break; i = 0; }
        buf[total++] = buf[i++];
        if (total > n) break;
    }
    return total;
}
int main(int argc, char **argv) { char b[64]; return work2(b, argc); }
