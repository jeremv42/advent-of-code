#include <stdio.h>
#include <string.h>

int main()
{
  void *fp = fopen("input", "r");

  int depth = 0;
  int forward = 0;

  char action[32];
  int nb;
  while (fscanf(fp, "%s %d\n", action, &nb) != EOF)
    {
      if (strcmp(action, "forward") == 0)
	forward += nb;
      else if (strcmp(action, "up") == 0)
	depth -= nb;
      else if (strcmp(action, "down") == 0)
	depth += nb;
    }

  printf("%d\n", depth * forward);
  
  fclose(fp);
  return 0;
}
