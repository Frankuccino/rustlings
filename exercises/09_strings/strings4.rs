// Calls of this function should be replaced with calls of `string_slice` or `string`.

// I created an enum
enum StringType<'a> {
    Slice(&'a str), // add a lifetime parameter to enum
    Owned(String),
}
// Maybe I could extent this string type to add the static str

fn placeholder(input: StringType) {
    match input {
        StringType::Slice(s) => string_slice(s),
        StringType::Owned(s) => string(s),
    }
}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    placeholder(StringType::Slice("blue"));

    placeholder(StringType::Owned("red".to_string()));

    placeholder(StringType::Owned(String::from("hi")));

    placeholder(StringType::Owned("rust is fun!".to_owned()));

    placeholder(StringType::Owned("nice weather".into()));

    placeholder(StringType::Owned(format!("Interpolation {}", "Station")));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    placeholder(StringType::Slice(&String::from("abc")[0..1]));

    placeholder(StringType::Slice("  hello there ".trim()));

    placeholder(StringType::Owned("Happy Monday!".replace("Mon", "Tues")));

    placeholder(StringType::Owned("mY sHiFt KeY iS sTiCkY".to_lowercase()));
}

// I created an enum-based abstraction with pattern matching dispatch instead of replacing the placeholder call.