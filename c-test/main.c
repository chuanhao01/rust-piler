#include <stdio.h>
typedef enum {
  OP_RETURN,
  OP_CONSTANT
} OpCode;


int main(int argc, const char* argv[]) {
  printf("%d", OP_RETURN);
  printf("\n");
  printf("%d", OP_CONSTANT);
  printf("\n");
  return 0;
}
