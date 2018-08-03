mod matrix;
mod process;

use matrix::Matrix;
use matrix::map::MapMatrix;

use process::Process;

fn main() {

	let num_states = 10;

	let mut process = Process {
		states: num_states,
		transition: MapMatrix::<f32>::new(0.0, num_states, num_states),
		reward: MapMatrix::<f32>::new(0.0, num_states, num_states),
		discount: 0.0
	};

	//0 can go to 1
	process.transition.set(0, 1, 1.0);

	//0 can go to 2
	process.transition.set(0, 2, 0.5);
	
	//1 and 2 can go to 0
	process.transition.set(1, 0, 1.0);
	process.transition.set(2, 0, 1.0);

	//Reward highly for going through 1, and poorly for going through 0
	process.reward.set(0, 1, 5.0);
	process.reward.set(0, 2, 1.0);

	println!("Transition Matrix");
	matrix::print_matrix(&process.transition);

	println!("Reward Matrix");
	matrix::print_matrix(&process.reward);

	//let mut reward = Vec::new();
}
