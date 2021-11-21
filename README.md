# Usage:

```rust
use wgpu_playground::winit::{
    event::WindowEvent, event_loop::ControlFlow, event_loop::EventLoop, window::Window,
};
use wgpu_playground::{wgpu, Spawner};

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    wgpu_playground::run::<App>(window, event_loop)
}

struct App {}

impl wgpu_playground::Playground for App {
    fn init(
        config: &wgpu::SurfaceConfiguration,
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Self {
        Self {}
    }

    fn resize(
        &mut self,
        config: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        // ...
    }

    fn update(&mut self, event: WindowEvent, control_flow: &mut ControlFlow) {
        if let WindowEvent::CloseRequested = event {
            *control_flow = ControlFlow::Exit;
        }
        // ...
    }

    fn render(
        &mut self,
        view: &wgpu::TextureView,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        spawner: &Spawner,
    ) {
        // ...
    }
}
```