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

void initialize_array(int arr[], int size) {
    register int i;
    for (i = 0; i < size; i++) arr[i] = size - i;
}

int main() {
    int count = CAP, number[CAP];
    initialize_array(number, count);
    quicksort(number, 0, count - 1);
    return 0;
}

