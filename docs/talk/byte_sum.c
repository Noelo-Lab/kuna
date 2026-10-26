__attribute__((noinline))
unsigned byte_sum(const unsigned char *bytes, unsigned length) {
    unsigned sum = 0;
    for (unsigned i = 0; i < length; ++i)
        sum += bytes[i];
    return sum;
}

int main(int argc, char **argv) {
    return (int)byte_sum((const unsigned char *)argv[0], (unsigned)argc);
}
