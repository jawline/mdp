extern crate rand;

mod matrix;
mod process;
mod markov;
mod greedy;
mod reinforcement;

use matrix::Matrix;
use matrix::map::MapMatrix;

use process::{State, Agent, Process};
use greedy::{ GreedyAgent };
use markov::{ MarkovChainAgent };
use reinforcement::{ ReinforcementAgent };

pub fn attempt<T: Agent<MapMatrix<f32>>>(process: &Process<MapMatrix<f32>>, step: &mut T) -> (usize, f32) {
	
	let mut state = State {
		node: 0,
		reward: 0.0
	};

	let mut rounds = 0;

	while process::step_process(&mut state, process, step) {
		rounds += 1;
	}

	(rounds, state.reward)
}

fn train_naive(process: &Process<MapMatrix<f32>>, num_states: usize) {
	let mut agent = ReinforcementAgent{ learned_reward: MapMatrix::<f32>::new(0.0, num_states, num_states) };

	let (first_run, reward_fr) = attempt(&process, &mut agent);

	for _ in 0..5 {
		attempt(&process, &mut agent);
	}

	let (last_run, reward_lr) = attempt(&process, &mut agent);

	println!("First vs Last {}, {} : {}, {}", first_run, reward_fr, last_run, reward_lr);
}

fn main() {

	let num_states = 10;

	let mut process = Process {
		states: num_states,
		transition: MapMatrix::<f32>::new(0.0, num_states, num_states),
		reward: MapMatrix::<f32>::new(0.0, num_states, num_states),
		discount: 0.0
	};

	//0 can go to 1
	process.transition.set(0, 1, 0.75);

	//0 can go to 2
	process.transition.set(0, 2, 0.25);
	
	//1 and 2 can go to 0
	process.transition.set(1, 0, 1.0);
	process.transition.set(2, 0, 0.60);
	process.transition.set(2, 1, 0.39);
	process.transition.set(2, 3, 0.01);

	//Reward highly for going through 1, and poorly for going through 0
	process.reward.set(0, 1, -1.0);
	process.reward.set(0, 2, -5.0);

	process.reward.set(1, 0, -10.0);

	process.reward.set(2, 0, 0.5);
	process.reward.set(2, 1, 1.0);
	process.reward.set(2, 3, 3.0);

	println!("Transition Matrix");
	matrix::print_matrix(&process.transition);

	println!("Reward Matrix");
	matrix::print_matrix(&process.reward);

	//attempt(&process, &mut GreedyAgent{});
	//attempt(&process, &mut MarkovChainAgent{});

	train_naive(&process, num_states);
}
