mod project;
mod rendering;
mod engine;
mod midi;

mod project1;

fn main() {
    rendering::run(project1::PROJECT.clone());
}

