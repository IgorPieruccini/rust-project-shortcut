use std::env;
use std::io;
use std::path::Path;
use std::process::Command;
mod prompt_options;
mod read_file;
mod save_to_file;

fn main() {
    let command_options: &Vec<&str> = &Vec::from(["open project", "add project shortcut"]);
    let path_to_projects: String = String::from("./projects_shortcut.txt");

    let selected_command = prompt_options::show(command_options);

    match selected_command {
        "open project" => {
            let mut input = String::new();
            println!("Choose a project:");
            let shortcut = read_file::read(&mut input);
            let selected_project = prompt_options::show(&shortcut);
            let root = Path::new(selected_project);
            assert!(env::set_current_dir(&root).is_ok());
            let result = Command::new("nvim").status();
            result.ok();
        }

        "add project shortcut" => {
            println!("insert the url of the project from the root of you system");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Could not read project url value");

            println!("Add project URL: {}", input);
            save_to_file::write(&input, &path_to_projects);
        }
        _ => println!("No commands found"),
    }
}
