#include <stdio.h>
#define CAP 100000

void quicksort(int number[CAP], int first, int last) {
    register int i, j, pivot, temp;
    if (first < last) {
        pivot = first;
        i = first;
        j = last;
        while (i < j) {
            while (number[i] <= number[pivot] && i < last) i++;
            while (number[j] > number[pivot]) j--;
            if (i < j) {
                temp = number[i];
                number[i] = number[j];
                number[j] = temp;
            }
        }
        temp = number[pivot];
        number[pivot] = number[j];
        number[j] = temp;
        quicksort(number, first, j - 1);
        quicksort(number, j + 1, last);
    }
}

void print_array(int arr[], int size) {
    register int i;
    for (i = 0; i < size; i++) printf("%d ", arr[i]);
    printf("\n");
}

void initialize_array(int arr[], int size) {
    register int i;
    for (i = 0; i < size; i++) arr[i] = size - i;
}

int main() {
    int count = CAP, number[CAP];
    initialize_array(number, count);
    printf("Before sorting");
    print_array(number, count);
    printf("\n");
//    printf("How many elements are u going to enter?: ");
//    scanf("%d", &count);
//    printf("Enter %d elements: ", count);
//    for (i = 0; i < count; i++) scanf("%d", &number[i]);
    quicksort(number, 0, count - 1);
    print_array(number, count);
//    printf("Order of Sorted elements: ");
//    for (i = 0; i < count; i++) printf(" %d", number[i]);
    return 0;
}

