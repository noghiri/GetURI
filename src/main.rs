//GetURI Rust Version
extern crate clipboard_win;
use std::env;
use clipboard_win::Clipboard;

fn main() {

    println!("Generating URI.");
    let generated_uri = generate_uri(); //calls the generate_uri function, and puts the result into a variable.
    println!("{}", generated_uri); //for debugging.
    Clipboard::new().unwrap().set_string(&generated_uri).expect("ERROR: Couldn't set clipboard contents");
}

fn generate_uri() -> String {
    let mut output_uri = String::from(r""); //Compiler complains about this, but without it, everything breaks. idk why.
    for arg in env::args().skip(1) {        //takes envarg in slot 1. slot 0 is the path to the executable itself. If you pass it multiple envargs, it'll cycle through - useful if you have an aftermarket clipboard. Someone should make sure that works.
        let mut uri_string = &arg;
        output_uri = uri_string.replace("\"", "/");
        output_uri = output_uri.replace(" ", "%20");  //these three lines transform something like \\server\the share\the program\program.exe to file://server/the%20share/the%20program/program.exe which works better for this application.
        output_uri.insert_str(0, "file:");
    }
    output_uri
}