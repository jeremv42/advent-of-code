#include <stdio.h>
#include <string.h>

int main()
{
  void *fp = fopen("input", "r");

  int nbDigits = 0;
  int totalNb = 0;
  char nb[32];
  int digits[32];
  memset(digits, 0, sizeof(digits));
  while (fscanf(fp, "%s", nb) != EOF)
    {
      int i = 0;
      while (nb[i])
	{
	  if (nb[i] == '1')
	    digits[i] += 1;
	  i += 1;
	}
      nbDigits = i;
      totalNb += 1;
    }

  int gamma = 0;
  int mask = 0;
  for (int i = 0; i < nbDigits; ++i)
    {
      if ((digits[i] * 1.0 / totalNb) >= 0.5)
	gamma = gamma | (1 << (nbDigits - i - 1));
      mask = mask | (1 << i);
    }

  int epsilon = (~gamma) & mask;
  printf("gamma = %d\nespilon = %d\nanswer = %d\n", gamma, epsilon, gamma * epsilon);

  fclose(fp);
  return 0;
}
