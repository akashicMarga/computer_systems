// Define a type for a pointer to an unsigned char
type BytePointer<'a> = &'a [u8];

// Function to print bytes of any data type
fn show_bytes(start: BytePointer) {
    for byte in start {
        print!(" {:02x}", byte);
    }
    println!();
}

// Function to show bytes of an integer
fn show_int(x: i32) {
    let bytes = x.to_ne_bytes();
    show_bytes(&bytes);
}

// Function to show bytes of a float
fn show_float(x: f32) {
    let bytes = x.to_ne_bytes();
    show_bytes(&bytes);
}

// Function to show bytes of a pointer
fn show_pointer<T>(x: &T) {
    let pointer: usize = x as *const T as usize;
    let bytes = pointer.to_ne_bytes();
    show_bytes(&bytes);
}

// Test function to demonstrate the byte representation of different data types
pub fn test_show_bytes(val: i32) {
    let ival = val; // Integer variable
    let fval = ival as f32; // Float variable initialized from int
    let pval = &ival; // Pointer to the integer variable
    show_int(ival); // Show bytes of integer
    show_float(fval); // Show bytes of float
    show_pointer(&pval); // Show bytes of pointer
}