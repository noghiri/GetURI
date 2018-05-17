//GetURI Rust Version
extern crate clipboard_win;
use std::env;
use clipboard_win::Clipboard;

fn main() {

    println!("Generating URI.");
    let generated_uri = generate_uri();
    println!("{}", generated_uri);
    Clipboard::new().unwrap().set_string(&generated_uri).expect("ERROR: Couldn't set clipboard contents");
}

fn generate_uri() -> String {
    let mut output_uri = String::from(r"");
    for arg in env::args().skip(1) {
        let mut uri_string = &arg;
        output_uri = uri_string.replace("\"", "/");
        output_uri = output_uri.replace(" ", "%20");
        output_uri.insert_str(0, "file:");
    }
    output_uri
}