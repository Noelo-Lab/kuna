namespace left { int target(int x) { return x + 10; } int (*getter(int target, int left))(int) { return left::target; } }
int main() { return left::getter(0,0)(7) == 17 ? 0 : 1; }
