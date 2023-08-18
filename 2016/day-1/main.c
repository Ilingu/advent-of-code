#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>

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

typedef struct
{
  int x;
  int y;
} Pos;

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

void updatePos(Pos *curr_pos, int curr_orientation)
{
  switch (curr_orientation)
  {
  case 0:
  case 360:
    (*curr_pos).x += 1;
    break;

  case 90:
    (*curr_pos).y += 1;
    break;

  case 180:
    (*curr_pos).x -= 1;
    break;

  case 270:
    (*curr_pos).y -= 1;
    break;

  default:
    exit(1);
    break;
  }
}

bool contains(Pos *seenPos, Pos search, unsigned int length)
{
  for (unsigned int i = 0; i < length; i++)
  {
    Pos curr = seenPos[i];
    if (curr.x == search.x && curr.y == search.y)
    {
      return true;
    }
  }
  return false;
}

int main()
{
  char *file_contents = getFile("input.txt");
  if (file_contents == NULL)
  {
    return EXIT_FAILURE;
  }

  // init position
  Pos curr_pos;
  curr_pos.x = 0;
  curr_pos.y = 0;
  int curr_orientation = 90;

  // init already seen position dynamic array
  unsigned int length = 0;
  Pos *seenPos = (Pos *)malloc(1 * sizeof(Pos));

  if (seenPos == NULL)
  {
    printf("Memory not allocated.\n");
    return EXIT_FAILURE;
  }

  // set the first seen position
  seenPos[0] = curr_pos;
  length++;

  int p2 = -1;

  // Split the instructions and interate over it
  char *token = strtok(file_contents, ", ");
  while (token != NULL)
  {
    // parse the number of block to move
    int numberOfBlocks = atoi(&token[1]);
    // update the orientation
    if (token[0] == 'R')
      curr_orientation = (curr_orientation + 270) % 360;
    else
      curr_orientation = (curr_orientation + 450) % 360;

    // update the position according to the current orientation and number of blocks to move
    for (int i = 0; i < numberOfBlocks; i++)
    {
      updatePos(&curr_pos, curr_orientation);

      // check if new pos is already seen
      if (p2 == -1 && contains(seenPos, curr_pos, length))
      {
        p2 = abs(0 - curr_pos.x) + abs(0 - curr_pos.y);
      }

      // push new seen pos
      seenPos = (Pos *)realloc(seenPos, (length + 1) * sizeof(Pos)); // add one to the capacity of the array
      if (!seenPos)
      {
        printf("Memory Re-allocation failed\n");
        free(file_contents);
        free(seenPos);
        return EXIT_FAILURE;
      }

      seenPos[length] = curr_pos; // push new pos
      length++;                   // update array length
    }

    // Retrieve next token
    token = strtok(NULL, ", ");
  }

  // Clean up
  free(file_contents);
  free(seenPos);

  int shortest_dist = abs(0 - curr_pos.x) + abs(0 - curr_pos.y);
  printf("p1: %d\n", shortest_dist);
  printf("p2: %d\n", p2);

  return EXIT_SUCCESS;
}