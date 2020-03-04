use std::collections::HashMap;
use std::io;

enum EmployeeDatabaseCommand {
    All,
    Add { name: String, department: String },
    Help,
}

impl EmployeeDatabaseCommand {
    fn from_input(command: &str) -> Option<Self> {
        let command_parts: Vec<&str> = command.split_whitespace().collect();
        match command_parts.as_slice() {
            ["All"] => Some(EmployeeDatabaseCommand::All),
            ["Add", name, "to", department] => Some(EmployeeDatabaseCommand::Add {
                name: (*name).to_string(),
                department: (*department).to_string(),
            }),
            ["Help"] => Some(EmployeeDatabaseCommand::Help),
            _ => None,
        }
    }
}

fn print_command_menu() {
    println!("Please enter a command. Valid options:");
    println!("- All");
    println!("- Add <employee_name> to <department>");
    println!("- Help");
}

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    print_command_menu();

    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        match EmployeeDatabaseCommand::from_input(&command) {
            Some(EmployeeDatabaseCommand::All) => {
                for (department, name_list) in departments.iter() {
                    println!("{}:", department);
                    let mut sorted_names = name_list.clone();
                    sorted_names.sort();
                    for name in sorted_names.iter() {
                        println!("- {}", name);
                    }
                }
            }
            Some(EmployeeDatabaseCommand::Add { name, department }) => {
                departments.entry(department).or_default().push(name);
            }
            Some(EmployeeDatabaseCommand::Help) => print_command_menu(),
            None => println!("Invalid command!"),
        }

        println!("\n");
        println!("Next command:");
    }
}
