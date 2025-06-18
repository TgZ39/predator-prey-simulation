run width height num_predators num_prey num_steps: build
    ./target/release/predator-prey-simulation {{width}} {{height}} {{num_predators}} {{num_prey}} {{num_steps}}
    just plot

build:
    cargo build --release

plot:
    julia plot.jl