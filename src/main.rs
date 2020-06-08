/*
*
*       TORAT (Treasury Office Routing number Analysis Tool)
*       Created by Sean Moore <sean.moore3@mail.mcgill.ca> on June 5th, 2020. 
*
*/

use std::fs::File; 
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::io::Write;
use std::env; 

struct Datum {
    routing_number : String, 
    state : String, 
    name : String, 
    address : String
}

//A help message for the user. 
const HELP_MESSAGE : &str = "
TORAT (Treasury Office Routing number Analysis Tool) Help Menu:
For issues or questions contact Sean Moore <sean.moore3@mail.mcgill.ca>.

Commands/Flags: 
help \tDisplay this help message.
-h \tDisplay this help message.
-f \tChange the output file.
-i \tChange the input file.
-s \tChange the state that we are filtering by.
-d \tChange the database file we are refrencing. 
-l \tSingle lookup mode. 

Examples: 
torat -h
torat -f out.txt -i target.txt
torat -s NH -d data.csv
torat -d data.csv -l #########

Default input file: target.txt.
Default output file: out.txt.
Default State: ME.
Default database file: data.csv.";

fn main() {
    //Get the input from the command line. 
    let mut args : Vec<String> = env::args().collect();

    //Set up default values for input/output. 
    let mut out_file = String::from("out.txt");
    let mut in_file = String::from("target.txt"); 
    let mut state = String::from("ME");
    let mut database_file = String::from("data.csv");

    //Parse the input and update the default values. 
    args.reverse();
    while args.len() > 0 {
        let arg = args.pop().unwrap();
        match arg.as_str() {
            "help" => {
                print_help_message();   //Print the help message.
                return;                 //Exit the program. 
            },
            "-h" => {
                print_help_message();   //Print the help message.
                return;                 //Exit the program. 
            },
            "-f" => {
                //Update the output file. 
                match args.pop() {
                    Some(file_path) => out_file = file_path,    //Update the path of the output file. 
                    None => {
                        println!("Please supply path of output file after -f flag.");
                        return; 
                    }
                } 
            },
            "-i" => {
                //Update the input file. 
                match args.pop() {
                    Some(file_path) => in_file = file_path,     //Update the path of the input file. 
                    None => {
                        println!("Please supply path of input file after -i flag.");
                        return; 
                    }
                }
            },
            "-s" => {
                //Update the input file. 
                match args.pop() {
                    Some(new_state) => state = new_state,       //Update the filter state. 
                    None => {
                        println!("Please supply the state to filter by after the -s flag.");
                        return; 
                    }
                }
            },
            "-d" => {
                //Update the input file. 
                match args.pop() {
                    Some(new_file) => database_file = new_file, //Update the filter state. 
                    None => {
                        println!("Please supply the state to filter by after the -s flag.");
                        return; 
                    }
                }
            },
            "-l" => {
                //Single lookup mode. 
                match args.pop() {
                    Some(rn) => {
                        single_lookup(rn, database_file);       //Look up a single routing number. 
                        return; 
                    }
                    None => {
                        println!("Please supply the routing number to look up after the -l flag.");
                        return; 
                    }
                }
            },
            _ => (), //Do nothing if we don't recognise the input as a command. 
        }
    }
    filter(in_file, out_file, state, database_file);
}

//Read each of the lines of the file and determine if they are from the state we want save the output. 
fn filter(input_path : String, output_path : String, state : String, records_path : String){
    //Load the csv into a hash map. 
    let data_in = File::open(records_path.clone());
    let data_in = match data_in {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Unable to open the database file {}.\n {:?}", records_path, error);
            return; 
        } 
    };
    let mut data : HashMap<String, Datum> = HashMap::new(); 
    for line in BufReader::new(data_in).lines(){
        let rec : String = line.unwrap(); 
        let rec : Vec<&str> = rec.split(",").collect::<Vec<_>>(); 
        let mut address = String::from(rec[2]);
        address += " ";
        address += rec[3]; 
        address += " "; 
        address += rec[4];
        address += " ";
        address += rec[5];
        let rec = Datum {
            routing_number : String::from(rec[0]), 
            address,
            name : String::from(rec[1]),
            state : String::from(rec[4]),
        };
        data.insert(rec.routing_number.clone(), rec);
    }

    let file_in = File::open(input_path.clone()); 
    let file_in = match file_in {
        Ok(file) => file, 
        Err(error) => {
            eprintln!("Unable to open input file {}.\n {:?}", input_path, error);
            return; 
        }
    };
    let out_file = File::create(output_path.clone());
    let mut out_file = match out_file {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Unable to open or create output file {} ({}.)",  output_path, error);
            return; 
        }
    };
    let mut unknowns : Vec<String> = Vec::new(); 
    for line in BufReader::new(file_in).lines(){
        let rn = line.unwrap(); 
        match data.get(&rn) {
            Some(bank) =>{
                //See if it is in maine. 
                if bank.state == state {
                    //Do nothing this entry is not suspicious. 
                } else {
                    //Save this enty to a file. 
                    let out : String = bank.routing_number.clone() + ":" + &bank.name + "(" + &bank.state + ")\n";
                    match out_file.write_all(out.as_bytes()) {
                        Ok(_) => (),
                        Err(_) => {
                            eprintln!("Error writing to output file.");
                            return; 
                        }
                    }
                }
            },
            None => {
                println!("Unknown routing number entered: {}.", rn);
                let uk = rn + ": unknown (not in database.)";
                unknowns.push(uk); 
            }
        }
    }
    //Add all of the unknows entries to the bottom of the output file. 
    for i in unknowns{
        match out_file.write_all(i.as_bytes()) {
            Ok(_) => (),
            Err(_) =>{
                println!("Error writing to output file.");
                return; 
            } 
        }
    }
} 

//Lookup a single record in the database and output information about it. 
fn single_lookup(routing_number : String, records_path : String){
    let data_in = File::open(records_path.clone());
    let data_in = match data_in {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Unable to open the database file {}.\n {:?}", records_path, error);
            return; 
        } 
    };
    for line in BufReader::new(data_in).lines(){
        let rec : String = line.unwrap(); 
        let rec : Vec<&str> = rec.split(",").collect::<Vec<_>>(); 
        let rn = String::from(rec[0]);
        if routing_number == rn {
            let mut address = String::from(rec[2]);
            address += " ";
            address += rec[3];
            address += " ";
            address += rec[5];
            println!("{}: {}, {} {}", routing_number, String::from(rec[1]), address, String::from(rec[4]));
            return; 
        }
    }
    println!("Unable to locate routing number ({}) in database.", routing_number);
}

//Print a help message to the screen.
fn print_help_message(){
    println!("{}", HELP_MESSAGE);
}
