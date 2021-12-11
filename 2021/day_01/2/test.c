#include <stdio.h>

int main() {
  void* fp = fopen("input", "r");

  int numbers[2000];
  int nb;
  int nbIdx = 0;
  while (fscanf(fp, "%d", &nb) != EOF)
    numbers[nbIdx++] = nb;
  fclose(fp);

  printf("%d, first=%d, last=%d\n", nbIdx, *numbers, numbers[1999]);

  int cc = 0;
  for (int i = 0; i < 2000 - 3; ++i)
  {
    int a = numbers[i + 0] + numbers[i + 1] + numbers[i + 2];
    int b = numbers[i + 1] + numbers[i + 2] + numbers[i + 3];
    if (a < b)
      cc += 1;
    printf("%d < %d ? cc=%d -- i=%d\n", a, b, cc, i);
  }
  printf("%d\n", cc);
  return 0;
}
