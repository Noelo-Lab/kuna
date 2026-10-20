#ifndef LOCAL_STACK_VIEWS_H
#define LOCAL_STACK_VIEWS_H

struct LifetimePair;
struct LifetimeHandle;

#ifndef LOCAL_STACK_VIEWS_FORWARD_TYPES
struct LifetimePair { int x; int y; };
struct LifetimeHandle { int *value; };
#endif

extern int datum;
extern int *escaped_pointer;

int read_pair(const struct LifetimePair *pair);
int read_handle(const struct LifetimeHandle *handle);
int read_escaped(void);
int read_saved_handle(void);
int same_address(const struct LifetimePair *pair, const struct LifetimeHandle *handle);
unsigned long long read_range_pair(const struct LifetimePair *pair);
unsigned long long read_range_handle(const struct LifetimeHandle *handle);
void write_saved(void);
void set_seven(int *value);

int local_constant(void);
int local_register(int a, int b);
int local_partial(void);
int local_branch(int condition);

#endif
