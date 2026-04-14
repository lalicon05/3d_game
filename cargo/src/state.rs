// This file will act as a core for the renderer


// Currenlty this is just me following the tutorial and will change in the future.
use std::sync::Arc;

use winit::{application::ApplicationHandler, event::*, event_loop::{ActiveEventLoop, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::{Window}};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;

pub struct State {
	window: Arc<Window>,
}

impl State {
	// According to the tutorial this doesn't need to be async yet, but will be in the next tutorial
	pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
		Ok(Self{window,})
	}

	pub fn resize(&mut self, _width: u32, _height: u32) {
		// For next tutorial
	}

	pub fn render(&mut self) {
		self.window.request_redraw();
		// More stuff will be added in the next tutorial
	}
}
// The tutorial continues in app.rs