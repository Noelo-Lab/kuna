int counter;

void bump(int n) { counter += n; }

void bump_twice(int n) {
    bump(n);
    bump(n * 2);
}

int bump_and_read(int n) {
    bump(n);
    return counter;
}
