/* The second, DIFFERENT `struct Same`: 16 bytes, four members. Same tag name as
 * the 4-byte one in dwarfstructs_x86_64.c -- legal across translation units, and
 * exactly the shape rustc produces for every enum payload struct (`Some`, `Ok`).
 */
#define NI __attribute__((noinline))
struct Same { long first; int second; int third; };
NI int read_same_big(struct Same *s) { return (int)s->first + s->second + s->third; }
