// Will be the 3d renderers updating loop and take inputs for basic camera controll and rendering option

// The tutorial continues from state.rs here
pub struct App {
	#[cfg(target_arch = "wasm32")]
	proxy: option<winit::event_loop::EventLoopProxy<State>>,
	state: option<State>,
}

impl App {
	pub fn new(#[cfg(target_arch = "wasm32")] event_loop: &EventLoop<State>) -> Self {
		#[cfg(target_arch = "wasm32")]
		let proxy = Some(event_loop.create_proxy());
		self {
			state: None,
			#[cfg(target_arch = "wasm32")]
			proxy,
		}
	}
}

// I will end here for today, 