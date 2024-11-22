use std::process;
use std::io;
use rand::Rng;
mod helper_fn;
use helper_fn::*;

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
        println!("\nSee you!");
        process::exit(0);
    } else if choice == 3 {
        println!("Returning to the previous menu...");
        return Err(());
    }
    Ok(())
}

fn show_result(folder: i32, problem_num: i32) -> i8 {
    let image_path = format!("problems/{}/{}_solve", folder, problem_num);
    display_img(&image_path, 80);
    result_options();
    let choice = read_input(4).try_into().unwrap();
    let _ = handle_choice(choice);
    
    if choice == 1 {
        return 1;
    }
    return 0;
}

fn problem(folder: i32, problem_num: i32) -> i8 {
    problem_options();
    let choice = read_input(3);
    if choice == 2 {
        println!("\nSee you!");
        process::exit(0);
    } else if choice == 1 {
        let res = show_result(folder, problem_num);
        println!("");
        return res;
    }
    return 0;
}

fn sub_menu(folder: i32) {
    let mut res = 0;
    let mut total = 0;
    let image_path = format!("formulas/{}", folder);
    loop {
        sub_menu_options();
        let choice = read_input(4);
        if choice == 4 {
            println!("\nSee you!");
            process::exit(0);
        } else if choice == 3 {
            println!("");
            return;
        } else if choice == 1 {
            display_img(&image_path, 60);
        } else {
            println!("");
            let mut rng = rand::thread_rng();
            let problem_num: i32 = rng.gen_range(1..=7);
            let image_path = format!("problems/{}/{}", folder, problem_num);
            display_img(&image_path, 80);
            res += problem(folder, problem_num);
            total += 1;
            println!("Your current result is {}/{}\n", res, total);
        }
    }
}

fn main() {
    display_img("matexpert", 30);
    println!("MaTeXpert! Your math teacher\n");
    loop {
        main_menu_options();
        let mut folder = read_input(4).try_into().unwrap();
        if folder == 2 || folder == 3 {
            println!("More problems coming soon! For now: trygonometry");
            folder = 1;
        }
        if folder == 4 {
            println!("\nSee you!");
            std::process::exit(0);
        }
        println!("");
        sub_menu(folder);
    }
}
