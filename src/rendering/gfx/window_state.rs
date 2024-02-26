
extern crate glfw;

use crate::rendering::sys::window_handler::{WindowHandler};
use glfw::{WindowEvent};

pub struct WindowState<'core> {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: (u32, u32),
    window: &'core WindowHandler
}

impl <'core> WindowState<'core> {
    async fn new(window: &'core WindowHandler) -> WindowState<'static> {
        todo!()
    }

    pub fn window(&self) -> &glfw::PWindow {
        &self.window.window()
    }

    fn resize(&mut self, new_size: (u32, u32)) {
        todo!()
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}