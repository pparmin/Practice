#include <stdio.h>
#define SIZE 20

void fill_numbers (int numbers[], int len) {
  for (int i = 0; i < len; i++) {
    numbers[i] = i+1;
    // printf("number: %u\n", i);

  }

}

int main() {
  int numbers[SIZE];
  int len = sizeof(numbers) / sizeof(int);
  int sum = 0;
  int sum_15 = 0;
  int result = 0;

  fill_numbers(numbers, SIZE);

  for (int i = 0; i < SIZE; i++) {
    // if ((numbers[i] % 15) == 0) {
    //   printf("Multiple of 15 found: %d at position %d\n", numbers[i], i);
    //   sum_15 += numbers[i]; 
    //   printf("Current sum of 15s: %d\n", sum_15);
    // } 
    
    if ((numbers[i] % 5) == 0) {
      printf("Multiple of 5 found: %d at position %d\n", numbers[i], i);
      sum += numbers[i]; 
      printf("Current sum of 5s: %d\n", sum);

    } else if ((numbers[i] % 3) == 0) {
      printf("Multiple of 3 found: %d at position %d\n", numbers[i], i);
      sum += numbers[i];
      printf("Current sum of 3s: %d\n", sum);

    } 
  }
  printf("Sum total: %d\n", sum);
  return 0;
}