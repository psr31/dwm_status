use std::process::Command;
use chrono::offset::Local;
use chrono::Datelike;

fn set_status(status: &str) {
    Command::new("xsetroot")
        .arg("-name")
        .arg(status)
        .output().expect("Failed to run xsetroot");
}

fn to_ordinal_string(number: u32) -> String {
    let str_num = number.to_string();

    let ordinal = match number % 10 {
        0 => "",
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    str_num + ordinal
}

fn main() {
    loop {
        let time = Local::now();
        let strf = format!("  %A {} of %B %G - %I:%M %p  ", to_ordinal_string(time.day()));
        let time_str = time.format(&strf).to_string();
        set_status(&time_str);
        std::thread::sleep(std::time::Duration::from_secs_f32(0.5));
    }
}
