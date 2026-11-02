/* option mipsunaligned: unaligned word loads and stores (GH-904). */
struct __attribute__((packed)) rec { char tag; int word; short half; };
int get_word(struct rec *r) { return r->word; }
void put_word(struct rec *r, int v) { r->word = v; }
void copy_word(struct rec *d, struct rec *s) { d->word = s->word; }
