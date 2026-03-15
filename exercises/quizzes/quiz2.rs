// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function as described above.
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String>{ 
        input
            .into_iter()
            .map(|(string, command)| {
                match command {
                    Command::Uppercase => string.to_uppercase(),

                    Command::Trim => string.trim().to_string(),

                    Command::Append(n) => {
                        let mut result = string;
                        for _ in 0..n {
                            result.push_str("bar")
                        }
                        result
                    }
                }
            })
            .collect()
     }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}

// The methods used in this quiz are the following, with explanation.
// We have a Vector:
// We use the .into_inter() method which converts the vector into an iterator that owns its elements. This means that we won't have borrowed references which in turn would not allows us to modify or consume the String
// The input was a Vec<(String, Command)> then the output iterator items would be (String, Command) -> This is because we want to move the String out and modify it.

// But if we used .iter() then each item would be &(String, Command) which means borrowed references and we wouldn't be able to modify or consume the String.

// Vec -> Iterator of Owned elements -> Processing pipeline

// The map() method transforms each element of the iterator into something else.
// Signature goes like Iterator<T> -> Iterator<U>
// vec![1,2,3].into_iter().map(|x| x * 2)
// become Iterator(<2,4,6>)

// Inside map: Tuple Destructuring
// |(string, command)| instead of |tuple| tuple.0 tuple.1

// match 
// Used to pattern match the enum variant, which is the Command enum
// and for each Command enum Trait we return the expected output value. Hence, a String.
