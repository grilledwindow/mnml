use image::Rgb;
mod icon;
use icon::IconShape::*;

fn main() {
    /*
    for lines in &all.split('\n').collect::<Vec<&str>>() {
        let arr = &lines.split(',').collect::<Vec<&str>>();
        let brr = arr[0].split(": ").collect::<Vec<&str>>();
        print!("let {} = [\"{}\", ", brr[0], brr[1]);
        for &color in &arr[1..] {
            print!("\"{}\", ", color);
        }
        println!("];");
    } */
    let green = ["facetime", "messages", "phone", "whatsapp", "grab", "citymapper", "spotify", "snapseed"].to_vec();
    // let blue = ["weather", "app store", "files", "telegram", "safari", "paypal", "carbonate"].to_vec();
    // let red = ["brave", "youtube", "bear", "netflix", "carousell", "ocbc", "pay anyone", "music", "mcdonald's"].to_vec();
    // let black = ["clock", "wallet", "stockes", "translate", "voice", "compass", "measure"].to_vec();
    // let grey = ["settings", "camera", "maps", "contacts", "calculator", "find my"].to_vec();
    // let white = ["calendar", "photos", "google photos", "reminders", "health", "notes", "lichess", "vsco"].to_vec();
    // let brown = ["bible"].to_vec();
    // let yellow = ["tips"].to_vec();
    // let orange = ["strava", "comet", "moovit"].to_vec();
    // let purple = ["itunes", "instagram"].to_vec(); 
    icon::create(&String::from("citymapper"), Rgb([150, 223, 211]), Circle, 900);
    // icon::create("phone", Rgb([150, 223, 211]), Rect, (900, 900));

    icon::create_same_colored_icons(&green, Rgb([150, 223, 211]));
    // create_same_colored_icons(blue, Rgb([154, 190, 255]));
    // create_same_colored_icons(red, Rgb([232, 94, 92]));
    // create_same_colored_icons(black, Rgb([86, 89, 91]));
    // create_same_colored_icons(grey, Rgb([160, 165, 175]));
    // create_same_colored_icons(white, Rgb([216, 216, 216]));
    // create_same_colored_icons(brown, Rgb([132, 101, 86]));
    // create_same_colored_icons(yellow, Rgb([233, 225, 197]));
    // create_same_colored_icons(orange, Rgb([248, 193, 140]));
    // create_same_colored_icons(purple, Rgb([189, 154, 255]));
}