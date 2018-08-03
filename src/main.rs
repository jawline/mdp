extern crate rand;

mod matrix;
mod process;
mod chain;

use matrix::Matrix;
use matrix::map::MapMatrix;

use process::{ProcessState, Process};
use chain::{ MarkovProcess, MarkovStep };

fn main() {

	let num_states = 10;

	let mut process = MarkovProcess {
		states: num_states,
		transition: MapMatrix::<f32>::new(0.0, num_states, num_states),
		reward: MapMatrix::<f32>::new(0.0, num_states, num_states),
		discount: 0.0
	};

	let mut step = MarkovStep {};

	let mut state = ProcessState {
		node: 0,
		reward: 0.0
	};

	//0 can go to 1
	process.transition.set(0, 1, 0.75);

	//0 can go to 2
	process.transition.set(0, 2, 0.25);
	
	//1 and 2 can go to 0
	process.transition.set(1, 0, 1.0);
	process.transition.set(2, 0, 1.0);

	//Reward highly for going through 1, and poorly for going through 0
	process.reward.set(0, 1, -1.0);
	process.reward.set(0, 2, 5.0);

	println!("Transition Matrix");
	matrix::print_matrix(&process.transition);

	println!("Reward Matrix");
	matrix::print_matrix(&process.reward);

	while process::step_process(&mut state, &process, &step) {}
}
