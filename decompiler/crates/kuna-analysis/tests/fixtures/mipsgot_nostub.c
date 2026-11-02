/* Imports with no lazy stub: an address-taken function and a weak reference. */
extern int puts(const char *);
extern void hook(void) __attribute__((weak));
int (*say_ptr)(const char *) = puts;
int user(const char *s) {
  puts(s);
  if (hook) hook();
  return say_ptr(s) + 1;
}
