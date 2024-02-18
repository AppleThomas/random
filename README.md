## A Rust Process Scheduler
COP4600 Assignment #1


This repo contains all of the code for running the following schedulers:
- First Come, First Serve (`fcfs`)
- Preemptive Shortest Job First (`sjf`)
- Non preemptive Shortest Job First (`realSJF`)
- Round-Robin (`rr`)

Note that string in `code` format is the shortened name used for the input files.

## Installation / Running
First, make sure to have rust and cargo installed, if not, install them using [rustup](https://rustup.rs).

Next, simply clone this repo to a desired location, and run the following command:
```
cargo run <inputfile.in>
```
Replace the `<inputfile.in>` section with the path to any input file you'd like to use.
