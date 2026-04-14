// This file will act as a core for the renderer


// Currenlty this is just me following the tutorial and will change in the future.
use std::sync::Arc;

use winit::{application::ApplicationHandler, event::*, event_loop::{ActiveEventLoop, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::{Window}};

struct State {
	window: Arc<Window>,
}

