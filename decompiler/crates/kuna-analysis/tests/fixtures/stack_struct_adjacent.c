struct pair { short x, y; };
void use_pair(struct pair *p);
void use_int(int *p);

int adjacent(void)
{
    struct pair a = {0};
    int b = 0;
    use_pair(&a);
    use_int(&b);
    return a.x + a.y + b;
}
