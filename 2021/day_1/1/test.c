#include <stdio.h>

int main() {
  void* fp = fopen("input", "r");

  int last = -1;
  int nb = 0;
  int cc = 0;
  fscanf(fp, "%d", &last);
  while (fscanf(fp, "%d", &nb) != EOF)
  {
    if (nb > last)
      cc += 1;
    last = nb;
  }
  printf("%d\n", cc);
  return 0;
}
