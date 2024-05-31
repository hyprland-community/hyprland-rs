/// Demostrates how to fetch and set keywords
/// 
/// Usage: cargo run --example keyword <keyword> <value>
/// Example: cargo run --example decoration:rounding (prints value) 
/// Example: cargo run --example decoration:rounding  15 (sets value)

use hyprland::keyword::Keyword;

fn main() -> hyprland::Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let keyword = if args.len() == 0 { "general:border_size" } else { &args[0].as_str() };
    let value = if args.len() > 1 { Some(&args[1]) } else { None };

    match value {
        // Here we change a keyword, in this case we are doubling the border size we got above
        Some(val) => Keyword::set(keyword, &**val)?,
        None => println!("{} value is {}", keyword, Keyword::get(keyword)?.value)
    }

    return Ok(());
}