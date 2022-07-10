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
                    println!("Adding {} to {}", &name, &department);
                    let dept_employees = employees_by_department
                        .entry(department)
                        .or_insert(Vec::new());
                    dept_employees.push(name);
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

fn sanitize_input<'a>(input: String) -> Result<Command, &'a str> {
    let trimmed_input: &str = input.trim();
    if trimmed_input.len() == 0 {
        return Err("No input given. To exit type `Exit`");
    }
    let trimmed_input: String = to_title_case(trimmed_input);
    let mut words: VecDeque<&str> = trimmed_input.split(' ').collect();
    let command = words.pop_front().unwrap();
    let parameter_string = space_join(words);
    if command.eq("Exit") {
        return Ok(Command::Exit);
    } else if command.eq("Get") {
        if parameter_string.len() == 0 {
            return Err("GET Command must be formatted as: GET All or GET <DEPARTMENT>");
        } else {
            return Ok(Command::Get(parameter_string));
        }
    } else if command.eq("Add") {
        let parameters: Vec<&str> = parameter_string.split(" To ").collect();
        if parameters.len() != 2 {
            return Err("Add Command must be formatted as: Add <EMPLOYEE> to <DEPARTMENT>");
        } else {
            return Ok(Command::Add(
                parameters[0].to_owned(),
                parameters[1].to_owned(),
            ));
        }
    } else if command.eq("Remove") {
        let parameters: Vec<&str> = parameter_string.split(" From ").collect();
        if parameters.len() != 2 {
            return Err("Remove Command must be formatted as: Remove <EMPLOYEE> from <DEPARTMENT>");
        } else {
            return Ok(Command::Remove(
                parameters[0].to_owned(),
                parameters[1].to_owned(),
            ));
        }
    } else {
        return Err("Command not found");
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

fn to_title_case(s: &str) -> String {
    let words: Vec<&str> = s.split(' ').collect();
    let mut response = String::new();
    for word in words {
        response.push_str(&word[0..1].to_uppercase());
        response.push_str(&word[1..].to_lowercase());
        response.push(' ');
    }

    return String::from(response.trim());
}
