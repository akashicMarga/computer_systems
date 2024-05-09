#include <stdio.h>

// Define a type for a pointer to an unsigned char
// typedef is used to create an alias for a data type. In this case, 'byte_pointer' is an alias for 'unsigned char *'.

typedef unsigned char *byte_pointer;

// Function to print bytes of any data type
void show_bytes(byte_pointer start, size_t len) {
    int i;
    for (i = 0; i < len; i++) {
        printf(" %.2x", start[i]); // Print each byte as two hexadecimal digits
    }
    printf("\n");
}

// Function to show bytes of an integer
void show_int(int x) {
    show_bytes((byte_pointer) &x, sizeof(int)); // Cast integer address to byte_pointer
}

// Function to show bytes of a float
void show_float(float x) {
    show_bytes((byte_pointer) &x, sizeof(float)); // Cast float address to byte_pointer
}

// Function to show bytes of a pointer
void show_pointer(void *x) {
    show_bytes((byte_pointer) &x, sizeof(void *)); // Cast pointer address to byte_pointer
}

// Test function to demonstrate the byte representation of different data types
void test_show_bytes(int val) {
    int ival = val; // Integer variable
    float fval = (float)ival; // Float variable initialized from int
    int *pval = &ival; // Pointer to the integer variable
    show_int(ival); // Show bytes of integer
    show_float(fval); // Show bytes of float
    show_pointer(pval); // Show bytes of pointer
}

// Main function to run the test
int main() {
    test_show_bytes(12345); // Test with the integer value 12345
    return 0;
}

