/* The fmt shape; the callee reads its third argument in its FIRST
   instruction, but its call-free body is longer than the entry probe's
   instruction budget, so the probe gives up and proves nothing. */
struct pair { long a; long b; };

__attribute__((noinline)) struct pair mk(long x)
{
    struct pair p = { x * 3, x ^ 0x55 };
    return p;
}

__attribute__((noinline)) long big3(int op, long a, long b)
{
    long h = b;
    h = h * 31 + (a >> 0) + 0;
    h = h * 33 + (a >> 1) + 1;
    h = h * 35 + (a >> 2) + 2;
    h = h * 37 + (a >> 3) + 3;
    h = h * 39 + (a >> 4) + 4;
    h = h * 41 + (a >> 5) + 5;
    h = h * 43 + (a >> 6) + 6;
    h = h * 45 + (a >> 7) + 7;
    h = h * 47 + (a >> 8) + 8;
    h = h * 49 + (a >> 9) + 9;
    h = h * 51 + (a >> 10) + 10;
    h = h * 53 + (a >> 11) + 11;
    h = h * 55 + (a >> 12) + 12;
    h = h * 57 + (a >> 0) + 13;
    h = h * 59 + (a >> 1) + 14;
    h = h * 61 + (a >> 2) + 15;
    h = h * 63 + (a >> 3) + 16;
    h = h * 65 + (a >> 4) + 17;
    h = h * 67 + (a >> 5) + 18;
    h = h * 69 + (a >> 6) + 19;
    h = h * 71 + (a >> 7) + 20;
    h = h * 73 + (a >> 8) + 21;
    h = h * 75 + (a >> 9) + 22;
    h = h * 77 + (a >> 10) + 23;
    h = h * 79 + (a >> 11) + 24;
    h = h * 81 + (a >> 12) + 25;
    h = h * 83 + (a >> 0) + 26;
    h = h * 85 + (a >> 1) + 27;
    h = h * 87 + (a >> 2) + 28;
    h = h * 89 + (a >> 3) + 29;
    h = h * 91 + (a >> 4) + 30;
    h = h * 93 + (a >> 5) + 31;
    h = h * 95 + (a >> 6) + 32;
    h = h * 97 + (a >> 7) + 33;
    h = h * 99 + (a >> 8) + 34;
    h = h * 101 + (a >> 9) + 35;
    h = h * 103 + (a >> 10) + 36;
    h = h * 105 + (a >> 11) + 37;
    h = h * 107 + (a >> 12) + 38;
    h = h * 109 + (a >> 0) + 39;
    h = h * 111 + (a >> 1) + 40;
    h = h * 113 + (a >> 2) + 41;
    h = h * 115 + (a >> 3) + 42;
    h = h * 117 + (a >> 4) + 43;
    h = h * 119 + (a >> 5) + 44;
    h = h * 121 + (a >> 6) + 45;
    h = h * 123 + (a >> 7) + 46;
    h = h * 125 + (a >> 8) + 47;
    h = h * 127 + (a >> 9) + 48;
    h = h * 129 + (a >> 10) + 49;
    h = h * 131 + (a >> 11) + 50;
    h = h * 133 + (a >> 12) + 51;
    h = h * 135 + (a >> 0) + 52;
    h = h * 137 + (a >> 1) + 53;
    h = h * 139 + (a >> 2) + 54;
    h = h * 141 + (a >> 3) + 55;
    h = h * 143 + (a >> 4) + 56;
    h = h * 145 + (a >> 5) + 57;
    h = h * 147 + (a >> 6) + 58;
    h = h * 149 + (a >> 7) + 59;
    h = h * 151 + (a >> 8) + 60;
    h = h * 153 + (a >> 9) + 61;
    h = h * 155 + (a >> 10) + 62;
    h = h * 157 + (a >> 11) + 63;
    h = h * 159 + (a >> 12) + 64;
    h = h * 161 + (a >> 0) + 65;
    h = h * 163 + (a >> 1) + 66;
    h = h * 165 + (a >> 2) + 67;
    h = h * 167 + (a >> 3) + 68;
    h = h * 169 + (a >> 4) + 69;
    h = h * 171 + (a >> 5) + 70;
    h = h * 173 + (a >> 6) + 71;
    h = h * 175 + (a >> 7) + 72;
    h = h * 177 + (a >> 8) + 73;
    h = h * 179 + (a >> 9) + 74;
    h = h * 181 + (a >> 10) + 75;
    h = h * 183 + (a >> 11) + 76;
    h = h * 185 + (a >> 12) + 77;
    h = h * 187 + (a >> 0) + 78;
    h = h * 189 + (a >> 1) + 79;
    h = h * 191 + (a >> 2) + 80;
    h = h * 193 + (a >> 3) + 81;
    h = h * 195 + (a >> 4) + 82;
    h = h * 197 + (a >> 5) + 83;
    h = h * 199 + (a >> 6) + 84;
    h = h * 201 + (a >> 7) + 85;
    h = h * 203 + (a >> 8) + 86;
    h = h * 205 + (a >> 9) + 87;
    h = h * 207 + (a >> 10) + 88;
    h = h * 209 + (a >> 11) + 89;
    return h + op;
}

__attribute__((noinline)) long caller(int c, long a, long d)
{
    struct pair p;
    if (c)
        p.a = a / d, p.b = a % d;
    else
        p = mk(a);
    return big3(c, p.a, p.b) + 1;
}

int main(int argc, char **argv)
{
    return (int)caller(argc & 7, (long)argv, argc + 1);
}
