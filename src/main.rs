use std::io::{stdin, stdout, Write};
use std::sync::{Arc,RwLock};
use std::string::String;
use crate::project::Project;

mod project;
mod rendering;
mod engine;
mod midi;

mod utils;
mod uniform_register;




mod project1;
mod project2;
mod project3;
mod project4;
mod project5;

fn main() {

    let projects :Vec<Arc<RwLock<Project>>> = vec![project1::PROJECT.clone(), project2::PROJECT.clone(), project3::PROJECT.clone(), project4::PROJECT.clone(),project5::PROJECT.clone()];

    if projects.len() == 1 {
        println!("Choosing the only available project: {}", projects[0].read().unwrap().name);
        rendering::run(projects[0].clone());
    } else if projects.len() > 1 {
        println!("Available projects:");
        for i in 0..projects.len() {
            println!("{} : {}",i,projects[i].read().unwrap().name);
        }

        print!("Please select project: ");
        let _ = stdout().flush();
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        let entry = input.trim().parse::<usize>().unwrap();
        if entry < projects.len()  {
            rendering::run(projects[entry].clone());
        } else {
            println!("Invalid project selected -_-");
        }

    } else {
        println!("No projects available :(");
    }
}

