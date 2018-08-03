use matrix::Matrix;

pub struct Process<T: Matrix<f32>> {
	pub states: usize,
	pub transition: T,
	pub reward: T,
	pub discount: f32
}

pub trait ProcessSteps<T: Matrix<f32>> {
	fn select_transfer(&self, possible: &Vec<(usize, f32)>) -> Option<usize>;
	fn try_transfer(&self, process: &Process<T>, from: usize, to: usize) -> bool;
	fn select_reward(&self, process: &Process<T>, from: usize, to: usize, success: bool) -> f32;
}

pub struct ProcessState {
	pub node: usize,
	pub reward: f32
}

pub fn step_process<T: Matrix<f32>, R: ProcessSteps<T>>(state: &mut ProcessState, process: &Process<T>, step: &R) -> bool {

	let mut transition_list = Vec::new();

	for i in 0..process.states {
		let probability = process.transition.get(state.node, i).unwrap();

		if probability != 0.0 {
			transition_list.push((i, probability));
		}
	}

	println!("Current State: {}", state.node);
	println!("Curren Reward: {}", state.reward);
	println!("Possible Transitions: ");

	for (state, probability) in &transition_list {
		println!("{}, {}", state, probability);
	}

	loop {
		if let Some(node) = step.select_transfer(&transition_list) {
			let transferred = step.try_transfer(&process, state.node, node);
			state.reward += step.select_reward(&process, state.node, node, transferred);
			
			if transferred {
				println!("Transferred to: {}", node);
				state.node = node;
				return true;
			}

		} else {
			return false;
		}
	}
}