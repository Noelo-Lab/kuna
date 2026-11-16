extern int optional(int) __attribute__((weak));
int counter;
int limit;
int guard(int value){counter+=value;return optional?optional(value):limit+7;}
int call_only(int value){return optional(value)+1;}
int same(int(*candidate)(int)){return candidate==optional;}
