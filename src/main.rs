use std::fs::File;
use std::io::Write;

fn main() {
    // todo: Handle errors correctly
    let mut file = File::create("todohunt.md").unwrap();
    file.write_all(b"- [ ] Buy milk.\n").unwrap();
    file.write_all(b"- [ ] Buy bread.\n").unwrap();
}
