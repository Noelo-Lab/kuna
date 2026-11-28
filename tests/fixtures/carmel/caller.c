extern int addone(int);
extern int stack_args(int, int, int, int, int, int, int, int, int);

int call_addone(int x)
{
    return addone(x);
}

int call_stack(int x)
{
    return stack_args(1, 2, 3, 4, 5, 6, 7, 8, x);
}

int call_indirect(int which, int value)
{
    int (*f)(int);
    if (which)
        f = addone;
    else
        return value;
    return (*f)(value);
}

int call_pointer(int (*f)(int), int value)
{
    return (*f)(value);
}
