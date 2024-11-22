use std::process::Command;

pub fn main_menu_options() {
    println!("Pick option:");
    println!("1 - trygonometry");
    println!("2 - polynomial");
    println!("3 - quadratic equasion");
    println!("4 - quit program");
}

pub fn sub_menu_options() {
    println!("Pick option:");
    println!("1 - show formulas");
    println!("2 - random problem");
    println!("3 - quit to main menu");
    println!("4 - quit program");
}

pub fn problem_options() {
    println!("Pick option:");
    println!("1 - another problem");
    println!("2 - show result");
    println!("3 - back");
    println!("4 - quit program");
}

pub fn result_options() {
    println!("Pick option:");
    println!("1 - mark as correct");
    println!("2 - mark as incorrect");
    println!("3 - back");
    println!("4 - quit program");
}

pub fn display_img(img: &str, width: i32) {
    //println!("{}", img);
    if cfg!(target_os = "linux") {
        let command = format!("viu {}.png -w {}", img, width);
        let status = Command::new("sh")
            .arg("-c")
            .arg(command)
            .status()
            .expect("failed to execute process");

        if status.success() {
            
        } else {
            eprintln!("Failed to display the image.");
        }
    } else {
        eprintln!("Linux, please!");
    }
}