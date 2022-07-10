use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::stdin;

fn main() {
    println!("Welcome to the employee database!");
    println!("The following commands are available:");
    println!("Add <EMPLOYEE> to <DEPARTMENT>");
    println!("Remove <EMPLOYEE> from <DEPARTMENT>");
    println!("Get <DEPARTMENT>");
    println!("Get All");
    println!("Exit");
    println!("Type your command:");

    let mut employees_by_department: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let command = sanitize_input(input);
        match command {
            Ok(x) => match x {
                Command::Exit => break,
                Command::Add(name, department) => {
                    let pd = department.clone();
                    let pn = name.clone();
                    let dept_employees = employees_by_department
                        .entry(department)
                        .or_insert(Vec::new());
                    dept_employees.push(name);
                    println!("Added {} to {}", pn, pd);
                }
                Command::Remove(name, department) => {
                    let dept_employees = match employees_by_department.get_mut(&department) {
                        Some(x) => x,
                        None => {
                            println!("No Such Department!");
                            continue;
                        }
                    };
                    let index = match dept_employees.iter().position(|x| *x == name) {
                        Some(x) => x,
                        None => {
                            println!("{} does not have an employee named {}", department, name);
                            continue;
                        }
                    };
                    dept_employees.remove(index);
                    println!("Removed {} from {}", name, department);
                }
                Command::Get(department) => {
                    if department.eq("All") {
                        let mut employees: Vec<String> = vec![];
                        for (dept, emps) in employees_by_department.iter() {
                            for emp in emps {
                                employees.push(format!("{} from {}", emp, dept));
                            }
                        }
                        employees.sort();
                        println!("{:?}", employees);
                    } else {
                        let dept_employees = match employees_by_department.get_mut(&department) {
                            Some(x) => x,
                            None => {
                                println!("No Such Department!");
                                continue;
                            }
                        };
                        dept_employees.sort();
                        println!("{:?}", dept_employees);
                    }
                }
            },
            Err(s) => {
                println!("{}", s);
                continue;
            }
        }
    }
}

enum Command {
    Add(String, String),
    Remove(String, String),
    Get(String),
    Exit,
}

fn sanitize_input(input: String) -> Result<Command, String> {
    let trimmed_input = String::from(input.trim());
    if trimmed_input.len() == 0 {
        return Err(String::from("No input given. To exit type `Exit`"));
    }
    let title_cased = to_title_case(trimmed_input);
    let mut words: VecDeque<&str> = title_cased.split(' ').collect();
    let command = words.pop_front().unwrap();
    let parameter_string = space_join(words);
    if command.eq("Exit") {
        return Ok(Command::Exit);
    } else if command.eq("Get") {
        if parameter_string.eq("") {
            return Err(String::from(
                "GET Command must be formatted as: GET All or GET <DEPARTMENT>",
            ));
        } else {
            return Ok(Command::Get(String::from(parameter_string)));
        }
    } else if command.eq("Add") {
        let parameters: Vec<&str> = parameter_string.split(" To ").collect();
        if parameters.len() != 2 {
            return Err(String::from(
                "Add Command must be formatted as: Add <EMPLOYEE> to <DEPARTMENT>",
            ));
        } else {
            return Ok(Command::Add(
                String::from(parameters[0]),
                String::from(parameters[1]),
            ));
        }
    } else if command.eq("Remove") {
        let parameters: Vec<&str> = parameter_string.split(" From ").collect();
        if parameters.len() != 2 {
            return Err(String::from(
                "Remove Command must be formatted as: Remove <EMPLOYEE> from <DEPARTMENT>",
            ));
        } else {
            return Ok(Command::Remove(
                String::from(parameters[0]),
                String::from(parameters[1]),
            ));
        }
    } else {
        return Err(String::from("Command not found"));
    }
}

fn space_join(words: VecDeque<&str>) -> String {
    let mut phrase = String::new();
    for word in words {
        phrase.push_str(word);
        phrase.push(' ');
    }
    phrase = String::from(phrase.trim());
    return phrase;
}

fn to_title_case(s: String) -> String {
    let words: Vec<&str> = s.split(' ').collect();
    let mut response = String::new();
    for word in words {
        response.push_str(&word[0..1].to_uppercase());
        response.push_str(&word[1..].to_lowercase());
        response.push(' ');
    }

    return String::from(response.trim());
}
