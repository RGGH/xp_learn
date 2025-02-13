use std::borrow::Cow;

fn process_string(input: &str) -> Cow<str> {
    // Check if the input string needs modification
    if input.contains(' ') {
        // If modification is needed, allocate a NEW STRING (Cow::Owned)
        let modified = input.replace(' ', "_");
        Cow::Owned(modified)
    } else {
        // Otherwise, just BORROW the input (Cow::Borrowed)
        Cow::Borrowed(input)
    }
}

fn main() {
    let original = "hello world";
    let processed = process_string(original);

    println!("Original: {}", original);   // "hello world"
    println!("Processed: {}", processed); // "hello_world"

    let already_good = "hello_world";
    let unprocessed = process_string(already_good);

    println!("Original: {}", already_good);   // "hello_world"
    println!("Unprocessed: {}", unprocessed); // "hello_world"
}

