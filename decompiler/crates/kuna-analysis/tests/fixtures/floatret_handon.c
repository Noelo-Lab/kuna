#include <string.h>
#include <stdio.h>
struct kt { const char *name; int type; int nid; };
static const struct kt keytypes[] = { {"ssh-rsa",0,0}, {"ssh-ed25519",3,0}, {"ecdsa",2,415}, {NULL,-1,-1} };
__attribute__((noinline)) const char *type_name(int type, int nid) {
  const struct kt *kt;
  for (kt = keytypes; kt->type != -1; kt++) if (kt->type == type && (kt->nid == 0 || kt->nid == nid)) return kt->name;
  return "ssh-unknown";
}
struct key { int type; int pad[5]; int nid; };
__attribute__((noinline)) int key_type_plain(int t) { return t & 7; }
__attribute__((noinline)) const char *key_ssh_name(const struct key *k) { return type_name(key_type_plain(k->type), k->nid); }
int main(int argc, char **argv) { struct key k = { argc + 2, {0}, 0 }; const char *n = key_ssh_name(&k); printf("%s %d\n", n, strcmp(n, "ssh-ed25519")); return 0; }
