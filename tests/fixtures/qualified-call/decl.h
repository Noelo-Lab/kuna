typedef int result_t;
typedef void (*callback_t)(unsigned int event);
result_t initialize(callback_t callback);
