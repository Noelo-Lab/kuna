/* Fixture for C declarator precedence in the declarations the printer builds
 * around a name: return types, locals and arrays of locals whose DWARF type
 * nests a pointer with an array. A pointer to an array needs its array suffix
 * AFTER the name (`char (*row)[16]`), which a type spelled only in front of the
 * name cannot express.
 *
 * Build:
 *   gcc -O2 -g -fno-stack-protector -fcf-protection=none -fno-inline \
 *       -o ptrarraydecl_x86_64 ptrarraydecl_x86_64.c
 */

static char rows[4][16];
static char *names[3] = { "a", "b", "c" };
char (*last_row)[16];
char *(*last_names)[3];

__attribute__((noinline)) char (*get_row(int i))[16]
{
	return &rows[i & 3];
}

__attribute__((noinline)) int use_rows(int i)
{
	char (*row)[16] = get_row(i);
	char (*pair[2])[16];

	pair[0] = row;
	pair[1] = get_row(i + 1);
	last_row = pair[i & 1];
	return (*row)[3] + (*pair[1])[2];
}

__attribute__((noinline)) char *(*get_names(void))[3]
{
	return &names;
}

__attribute__((noinline)) int use_names(int i)
{
	char *(*table)[3] = get_names();
	char *first[2];

	first[0] = (*table)[0];
	first[1] = (*table)[i % 3];
	last_names = table;
	return first[i & 1][0];
}

int main(int argc, char **argv)
{
	(void)argv;
	return use_rows(argc) + use_names(argc);
}
