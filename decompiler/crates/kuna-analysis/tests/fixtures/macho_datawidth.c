struct pt { int x; int y; };
struct pt origin = {1, 2};
int counter = 5;
__attribute__((noinline)) int bump(void) { counter += 1; origin.x += 1; return counter + origin.y; }
int main(void) { return bump(); }
