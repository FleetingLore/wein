use std::io;

fn main() {
    println!(
        r##"\
        <p style="margin-left: 0rem">\
        <a href="#" class="link-lore" data-path="0">\
        0\
        </a>\
        </p>"##);
    // let mut mode: String = "html".to_string();
    let mut from: String = "".to_string();
    let mut to: String = "".to_string();

    // println!("Select mode (html): ");
    // io::stdin().read_line(&mut mode).expect("Failed to read line");

    println!("from: ");
    io::stdin()
        .read_line(&mut from)
        .expect("Failed to read line");
    let from: String = from.trim().parse().expect("Failed to parse");
    
    println!("to: ");
    io::stdin()
        .read_line(&mut to)
        .expect("Failed to read line");
    let to: String = to.trim().parse().expect("Failed to parse");
    
    // TODO

    println!();
    println!("from {} to {}", from, to);
}
