mod project;
mod rendering;
mod engine;
mod midi;

mod utils;
mod project3;
mod uniform_register;

fn main() {
    rendering::run(project3::PROJECT.clone());
}

