#include <stdlib.h>
#include <string.h>
#include <stdio.h>

char *getFile(const char *__restrict__ __filename)
{
  FILE *file = fopen(__filename, "r");
  if (file == NULL)
  {
    printf("Failed to open the file.\n");
    return NULL; // Exit the program with an error
  }

  // Determine the file size
  fseek(file, 0, SEEK_END);
  long file_size = ftell(file);
  fseek(file, 0, SEEK_SET);

  // Allocate memory for the string
  char *file_contents = (char *)malloc(file_size + 1); // +1 for null terminator
  if (file_contents == NULL)
  {
    printf("Failed to allocate memory.\n");
    fclose(file);
    return NULL;
  }

  // Read the file into the string
  size_t bytes_read = fread(file_contents, 1, file_size, file);
  if (bytes_read != file_size)
  {
    printf("Failed to read the file.\n");
    free(file_contents);
    fclose(file);
    return NULL;
  }
  file_contents[file_size] = '\0';

  fclose(file);
  return file_contents;
}

const char separator[2] = "\n";

char *part1(char *file_contents)
{
  // init current position
  unsigned short padlockDigit = 4;

  // results
  char *p1 = (char *)malloc(0);
  if (p1 == NULL)
    return NULL;

  // iterate over intructions and update position
  unsigned int line = 0;
  char *token = strtok(file_contents, separator);
  while (token != NULL)
  {
    for (int i = 0; token[i] != '\0'; i++)
    {
      char direction = token[i];
      unsigned short line = padlockDigit / 3;
      switch (direction)
      {
      case 'U':
        if (padlockDigit - 3 >= 0)
          padlockDigit -= 3;
        break;
      case 'D':
        if (padlockDigit + 3 <= 8)
          padlockDigit += 3;
        break;
      case 'L':
        short tempL = padlockDigit - 1;
        if (tempL / 3 == line && tempL >= 0)
          padlockDigit = tempL;
        break;
      case 'R':
        short tempR = padlockDigit + 1;
        if (tempR / 3 == line && tempR <= 8)
          padlockDigit = tempR;
        break;
      default:
        return NULL;
        break;
      }
    }

    p1 = (char *)realloc(p1, (line + 1) * sizeof(char)); // add one to the capacity of the array
    if (!p1)
    {
      printf("Memory Re-allocation failed\n");
      free(p1);
      return NULL;
    }
    p1[line] = padlockDigit + 1 + '0';

    // Retrieve next token
    token = strtok(NULL, separator);
    line++;
  }

  return p1;
}

int main()
{
  char *file_contents = getFile("input.txt");
  if (file_contents == NULL)
  {
    return EXIT_FAILURE;
  }

  char *p1 = part1(file_contents);
  if (p1 == NULL)
  {
    return EXIT_FAILURE;
  }

  printf("%s\n", p1);

  // clean up
  free(file_contents);
  free(p1);

  return EXIT_SUCCESS;
}