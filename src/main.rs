use std::path::Path;
use std::io::{Write, BufRead, BufReader};
use std::fs::File;
use std::io;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};


fn get_max_float(floats: &[f64]) -> f64 {
    let mut max_value = floats[0];

    for &float in floats.iter().skip(1) {
        if float > max_value {
            max_value = float
        }
    }
    max_value
}

fn get_min_float(floats: &[f64]) -> f64 {
    let mut min_value = floats[0];

    for &float in floats.iter().skip(1) {
        if float < min_value {
            min_value = float
        }
    }
    min_value
}

fn import_file(path: &str) -> Result<Vec<f64>> {
    let mut import_vec: Vec<f64> = Vec::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut line_count = 0;

    for (_index, line) in reader.lines().enumerate() {
        line_count += 1;
        let number = line?.trim().to_string();

        if !number.is_empty() {
            if let Ok(num) = number.parse::<f64>() {
                import_vec.push(num);
            } else {
                eprintln!("Invalid numeric value found in line {}: {}", line_count, number);
                continue;
            }
        }
    }

    Ok(import_vec)
}


fn export_file(values: &[f64], path: &str) -> io::Result<()> {
    let path = Path::new(path);

    if path.exists() {
        // File already exists, prompt for overwrite
        print!("File '{}' already exists. Do you want to overwrite it? (y/n): ", path.display());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let answer = input.trim().to_lowercase();

        if answer != "y" && answer != "yes" {
            // User chose not to overwrite, return early
            println!("Export canceled.");
            return Ok(());
        }
    }

    let mut file = std::fs::File::create(path)?;
    
    for value in values {
        let value_string = value.to_string();
        file.write_all(value_string.as_bytes())?;
        file.write_all(b"\n")?;
    }
    
    println!("Values written to file '{}'.", path.display());
    Ok(())
}


fn main() -> Result<()> {
    println!("Enter a number and it will be added to the list");
    println!("Type \"help\" for list of commands");
    let mut user_inputs: Vec<f64> = Vec::new();

    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                let trimmed_input = line.trim();

                if !trimmed_input.is_empty() {
                    if trimmed_input == "exit" {
                        break;
                    }
                    else if trimmed_input == "ls" {
                        if user_inputs.is_empty() { println!("List empty") }
                        else { println!("{:?}", user_inputs); }
                    }
                    else if trimmed_input == "help" {
                        print_help()
                    }

                    else if trimmed_input.starts_with("im") {
                        let words = trimmed_input.split_whitespace().collect::<Vec<&str>>();
                        if let Some(path) = words.get(1) {
                            match import_file(path) {
                                Ok(numbers) => {
                                    println!("Imported numbers: {:?}", numbers);
                                    user_inputs.extend(numbers)
                                }
                                Err(error) => {
                                    eprintln!("Error importing file: {}", error);
                                }
                            }
                        } else {
                            println!("Invalid path")
                        }
                    }

                    else if !user_inputs.is_empty() {
                        if trimmed_input == "o" {
                            let mut sorted_user_inputs = user_inputs.clone();
                            sorted_user_inputs.sort_by(|a, b| a.partial_cmp(b).unwrap());
                            println!("{:?}", sorted_user_inputs)
                        }
                        else if trimmed_input == "ob" {
                            let mut sorted_user_inputs = user_inputs.clone();
                            sorted_user_inputs.sort_by(|a, b| b.partial_cmp(a).unwrap());
                            println!("{:?}", sorted_user_inputs)
                        }
                        else if trimmed_input == "clear" {
                            user_inputs.clear();
                        }
                        else if trimmed_input == "a" {
                            let sum: f64 = user_inputs.iter().sum();
                            println!("Total: {}", sum) 
                        }
                        else if trimmed_input == "m" {
                            let prod: f64 = user_inputs.iter().product();
                            println!("Product: {}", prod) 
                        }
                        else if trimmed_input == "ma" {
                            let sum: f64 = user_inputs.iter().sum();
                            let mean: f64 = sum / user_inputs.len() as f64;
                            println!("Mean: {}", mean)
                        }
                        else if trimmed_input == "ra" {
                            let max: f64 = get_max_float(&user_inputs);
                            let min: f64 = get_min_float(&user_inputs);
                            let range: f64 = max - min;
                            println!("Range: {}", range);
                        }
                        else if trimmed_input == "l" {
                            let max: f64 = get_max_float(&user_inputs);
                            println!("Largest number: {}", max);
                        }
                        else if trimmed_input == "s" {
                            let min: f64 = get_min_float(&user_inputs);
                            println!("Smallest number: {}", min);
                        }
                        else if trimmed_input == "len" {
                            let length = user_inputs.len();
                            println!("List lenght: {}", length);
                        }
                        else if trimmed_input.starts_with("rm") {
                            let mut parts = trimmed_input.split_whitespace();

                            if let Some(float_str) = parts.nth(1) {
                                if let Ok(float_val) = float_str.parse::<f64>() {
                                    println!("Removed: {}", float_val);
                                    if let Some(index) = user_inputs.iter().position(|value| *value == float_val) {
                                        user_inputs.swap_remove(index);
                                    }
                                }
                                else {
                                    println!("Not a valid number")
                                }
                            }
                            else {
                                println!("Enter a number after rm e.g. \"rm 300\"");
                            }
                        }

                        else if trimmed_input.starts_with("ex") {
                            let words = trimmed_input.split_whitespace().collect::<Vec<&str>>();

                            if let Some(path) = words.get(1) {
                                match export_file(&user_inputs, path) {
                                    Ok(_) => continue,
                                    Err(error) => eprintln!("Error exporting file: {}", error),
                                }
                            } else {
                                println!("Invalid path")
                            }                    
                        }


                        else {
                            let parsed_input = match trimmed_input.parse::<f64>() {
                                Ok(value) => value,
                                Err(_) => {
                                    println!("Invalid Input");
                                    continue; // Skip if parsing fails
                                }
                            };

                            user_inputs.push(parsed_input);

                        }
                    }
                    else {
                        let parsed_input = match trimmed_input.parse::<f64>() {
                            Ok(value) => value,
                            Err(_) => {
                                println!("Invalid Input");
                                continue; // Skip if parsing fails
                            }
                        };

                        user_inputs.push(parsed_input);
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    Ok(())
}

fn print_help() {
    let help_string = "\nTo add a number to the list just type it.

Commands to run on the list:

    ls - print list
    o - print ordered list

    rm - remove number from list e.g. \"rm 100\"
    clear - removes all numbers from the list

    ra - print range
    ma - print mean

    a - add all numbers in list
    m - multiply all numbers in list

    s - print smallest number in list
    l - print largest number in list

    im - import file with a number on each line e.g. \"im list.txt\"
    ex - export list e.g. \"ex list.txt\" saves a list.txt file

    exit - exit numlist\n"; 
    println!("{}", help_string)
}
