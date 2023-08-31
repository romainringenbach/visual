mod project;
mod rendering;
mod engine;
mod midi;

mod project2;
mod utils;

fn main() {
    rendering::run(project2::PROJECT.clone());
}

