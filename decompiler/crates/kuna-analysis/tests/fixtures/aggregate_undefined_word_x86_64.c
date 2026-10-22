/* gcc 15.2 (Ubuntu 26.04): gcc -O2 -g -march=x86-64 -fno-tree-vectorize -fno-pie -no-pie -DSAMPLE_MAIN aggregate_undefined_word_x86_64.c -o aggregate_undefined_word_x86_64 */
#include <string.h>
typedef struct reading {float pressure,temperature,altitude;} reading;
typedef struct sample {unsigned stamp;float distance,deviation;} sample;
typedef struct message {int kind;union {reading weather;sample range;int count[3];} data;} message;
int next_message(message *m);
reading latest;int last_count;int pending;message queue[4];
__attribute__((noinline)) int next_message(message *m) {
    if (pending <= 0) return 0;
    *m = queue[--pending];return 1;
}
float latest_weather(void) {
    message m;reading keep = latest;
    while (next_message(&m)) {
        if (m.kind == 1) keep = m.data.weather;
        else latest = keep;
    }
    latest = keep;return keep.pressure;
}
int latest_count(void) {
    message m;int count = last_count;float pressure = latest.pressure;
    while (next_message(&m)) {
        if (m.kind == 0) count = m.data.count[0];
        else if (m.kind == 1) pressure = m.data.weather.pressure;
    }
    last_count = count;latest.pressure = pressure;return count;
}
#ifdef SAMPLE_MAIN
int main(void) {return 0;}
#endif
