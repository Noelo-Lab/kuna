struct Rect { int x, y, w, h, flags, a, b, c; };
int __stdcall paint(int a, int b, int c);
int __stdcall fill(Rect r, int color);
int __stdcall draw(int a, int b, const Rect *src) {
    int n = paint(a, b, 3);
    Rect r = *src;
    r.x += n;
    return n + fill(r, b);
}
