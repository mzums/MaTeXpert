use std::process;
use std::io;
use rand::Rng;
mod helper_fn;
use helper_fn::{main_menu_options, sub_menu_options, problem_options, result_options, display_img};

fn read_input(max_option: u32) -> u32 {
    loop {
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim().parse::<u32>() {
            Ok(num) if (1..=max_option).contains(&num) => {
                return num;
            }
            _ => {
                println!("Pick an option 1-{}:", max_option);
            }
        }
    }
}

fn handle_choice(choice: i32) -> Result<(), ()> {
    if choice == 4 {
        println!("See you!");
        process::exit(0);
    } else if choice == 3 {
        println!("Returning to the previous menu...");
        return Err(());
    }
    Ok(())
}

fn show_result(folder: i32, problem_num: i32) {
    let image_path = format!("problems/{}/{}_solve", folder, problem_num);
    display_img(&image_path, 80);
    result_options();
    let choice = read_input(4).try_into().unwrap();
    let _ = handle_choice(choice);
}

fn problem(folder: i32, problem_num: i32) {
    problem_options();
    let choice = read_input(4).try_into().unwrap();
    let _ = handle_choice(choice);
    if choice == 2 {
        show_result(folder, problem_num);
    }
}

fn sub_menu(folder: i32) {
    while true {
        sub_menu_options();
        let choice = read_input(4).try_into().unwrap();
        let _ = handle_choice(choice);
        println!("");
        let mut rng = rand::thread_rng();
        let problem_num: i32 = rng.gen_range(1..=7);
        let image_path = format!("problems/{}/{}", folder, problem_num);
        display_img(&image_path, 80);
        problem(folder, problem_num);
    }
}

fn main() {
    while true {
        display_img("matexpert", 30);
        println!("MaTeXpert! Your math teacher\n");

        main_menu_options();
        let folder = read_input(4).try_into().unwrap();
        if folder == 4 {
            println!("See you!");
            std::process::exit(0);
        }
        println!("");
        sub_menu(folder);
    }
}
