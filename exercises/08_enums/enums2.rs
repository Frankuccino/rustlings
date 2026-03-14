#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

struct Dimensions {
    width: u64,
    height: u64,
} // This does not work for the Resize Variant of the Message Enum
// I just went ahead and made a struct-like variant for Resize variant.

#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    Resize{width: u64, height: u64}, // here -> Struct-like variant
    Move(Point), // these are tuple-struct variants
    Echo(String), // Just a tuple variant.
    ChangeColor(u32,u32,u32), // This is a tuple variants, multiple types
    Quit // This is a Unit Variant
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
