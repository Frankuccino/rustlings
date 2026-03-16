fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if-let statement whose value is `Some`.
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
// You can see at line 23, we used an inclusive range syntax:
// start..=end.   -- Which we iterate from the start and up to including the end.
// unlike in an exclusive range, we actually exclude the end.
// like 0..5, means we go 0,1,2,3,4

// We did an inclusive range syntax on line23 1..=10, to push the values 1 to 10 in the vector as a Some() Variant

// Then with the while let we actually pop every element on that vector to extract the values using Some(Some(value));
// The reason why it is a nested Some pattern match because each element in the vector is itself an Option.
// If the value is just simple a Some(value), we could extract it through just doing Some(value), but since the element in the vector itself is an Option