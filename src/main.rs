use serde::{Deserialize,Serialize};
use std::{fmt::format, fs::read_to_string};
use inquire::{Text,Select};
use std::process::Command;

#[derive(Deserialize,Serialize,Debug)]
struct AppConfig{
    project : Vec<Project>,
    mongodb : Vec<String>,
    db : Vec<String>,
    collection : Vec<String>
}

#[derive(Deserialize,Serialize,Debug)]
struct Project {
    customer : String,
    //#[serde(default = "default_path")] 
    branch : Vec<String>
}



fn main(){
    let ron_file = read_to_string("config.ron").expect("unablet to read the file");
    let ron_config : AppConfig = ron::from_str(&ron_file).expect("Something wrong with the run file");
    println!("my config from ron : {:?}",ron_config);

    let mut customer_option = ron_config.project.iter().map(|ele| ele.customer.clone()).collect::<Vec<String>>();
    customer_option.push("Other".to_string());

    println!("Welcome automated adder");

    let customer = Select::new("Enter the customer name",customer_option).with_help_message("Choose the coustomer name if not present choose other").prompt().unwrap();

    // println!("you have choose = {}",customer);

    let branch : String;

    if &customer == "Other"{
        branch = Text::new("Enter the branch").with_help_message("As you have choose other as customer name, please type the customer name").prompt().unwrap();
    }else{
        let mut branch_option = ron_config.project.iter().find(|ele| ele.customer == customer ).unwrap().branch.to_owned();
        branch_option.push("Other".to_string());

        let temp = Select::new("Enter the branch",branch_option).prompt().unwrap();
        if &temp == "Other"{
            branch = Text::new("Enter the branch").prompt().unwrap();
        }else{
            branch = temp;
        }
    }

    println!("you have choose = {}",branch);

}