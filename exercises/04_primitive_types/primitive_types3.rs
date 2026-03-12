fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a = [0; 100];
// This syntax is [expr; N]
// Where expr is the values you'll initialize the array
// Where N is the number of elements to be initialized.

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
