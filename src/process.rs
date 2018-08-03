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
}

pub struct ProcessState {
	pub current_state: usize
}

pub fn step_process<T: Matrix<f32>, R: ProcessSteps<T>>(state: &mut ProcessState, process: &Process<T>, step: &R) -> bool {

	let mut transition_list = Vec::new();

	for i in 0..process.states {
		let probability = process.transition.get(state.current_state, i).unwrap();

		if probability != 0.0 {
			transition_list.push((i, probability));
		}
	}

	println!("Current State: {}", state.current_state);
	println!("Possible Transitions: ");

	for (state, probability) in &transition_list {
		println!("{}, {}", state, probability);
	}

	loop {
		if let Some(node) = step.select_transfer(&transition_list) {
			if step.try_transfer(&process, state.current_state, node) {
				state.current_state = node;
				return true;
			}
		} else {
			return false;
		}
	}
}