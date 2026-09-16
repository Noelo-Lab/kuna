#include <stdio.h>
int main(int argc, char **argv, char **envp) {
  for (int i = 0; envp[i]; i++) puts(envp[i]);
  return argc;
}
