mod project;
mod rendering;
mod engine;
mod midi;

mod project2;

fn main() {
    rendering::run(project2::PROJECT.clone());
}

