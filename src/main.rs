use std::collections::BTreeMap;
use std::io::{stdin, stdout, Write};
use std::sync::{Arc, Mutex,RwLock};
use std::string::String;
use crate::project::Project;
use lazy_static::lazy_static;

mod project;
mod rendering;
mod engine;
mod midi;

mod utils;
mod uniform_register;




mod project1;
mod project2;
mod project3;

fn main() {

    let projects :Vec<Arc<RwLock<Project>>> = vec![project1::PROJECT.clone(), project2::PROJECT.clone(), project3::PROJECT.clone()];

    if projects.len() == 1 {
        println!("Choosing the only available project: {}", projects[0].read().unwrap().name);
        rendering::run(projects[0].clone());
    } else if projects.len() > 1 {
        println!("Available projects:");
        for i in 0..projects.len() {
            println!("{} : {}",i,projects[i].read().unwrap().name);
        }

        print!("Please select project: ");
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input);
        let entry = input.trim().parse::<usize>().unwrap();
        if(entry < projects.len()) {
            rendering::run(projects[entry].clone());
        } else {
            println!("Invalid project selected -_-");
        }

    } else {
        println!("No projects available :(");
    }
}

