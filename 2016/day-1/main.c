#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#define MAX(x, y) (((x) > (y)) ? (x) : (y))
#define MIN(x, y) (((x) < (y)) ? (x) : (y))

int abs(int n)
{
  return (n < 0) ? -n : n;
}

int clamp(int x, int y, int z)
{
  return MIN(MAX(x, y), z);
};

struct Pos
{
  int x;
  int y;
};

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

int main()
{
  char *file_contents = getFile("input.txt");
  if (file_contents == NULL)
  {
    return EXIT_FAILURE;
  }

  // init position
  struct Pos curr_pos;
  curr_pos.x = 0;
  curr_pos.y = 0;
  int curr_orientation = 90;

  // Split the instructions and interate over it
  char *token = strtok(file_contents, ", ");
  while (token != NULL)
  {
    int numberOfBlocks = atoi(&token[1]);
    if (token[0] == 'R')
      curr_orientation = (curr_orientation + 270) % 360;
    else
      curr_orientation = (curr_orientation + 450) % 360;

    switch (curr_orientation)
    {
    case 0:
    case 360:
      curr_pos.x += numberOfBlocks;
      break;

    case 90:
      curr_pos.y += numberOfBlocks;
      break;

    case 180:
      curr_pos.x -= numberOfBlocks;
      break;

    case 270:
      curr_pos.y -= numberOfBlocks;
      break;

    default:
      return EXIT_FAILURE;
      break;
    }

    token = strtok(NULL, ", "); // Retrieve next token
  }

  // Clean up
  free(file_contents);

  int shortest_dist = abs(0 - curr_pos.x) + abs(0 - curr_pos.y);
  printf("%d\n", shortest_dist);

  return EXIT_SUCCESS;
}