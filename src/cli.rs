use clap::Parser;

#[derive(Parser, Debug, Copy, Clone)]
pub struct Cli {
    pub width: usize,
    pub height: usize,
    pub num_predators: usize,
    pub num_prey: usize,
    pub num_steps: usize,
}
