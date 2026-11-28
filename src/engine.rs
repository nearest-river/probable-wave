

use std::{
  mem,
  sync::Arc,
};

use crate::{
  model::*,
  prelude::*,
  renderer::Renderer,
};

use winit::{
  dpi::PhysicalSize,
  application::ApplicationHandler,
  window::{
    Window,
    WindowId,
  },
  event_loop::{
    EventLoop,
    ActiveEventLoop,
  },
  event::{
    DeviceId,
    DeviceEvent,
    WindowEvent,
    MouseButton,
    ElementState,
    MouseScrollDelta,
  },
};



pub struct Engine {
  state: State,
  event_loop: EventLoop<()>
}

impl Engine {
  #[inline]
  pub fn new()-> Self {
    let event_loop=EventLoop::new()
    .unwrap();

    let state=State::default();

    Self {
      state,
      event_loop,
    }
  }

  #[inline(always)]
  pub fn with_title(mut self,title: impl Into<String>)-> Self {
    self.state.title=title.into();
    self
  }

  #[inline(always)]
  pub fn with_clear_color(mut self,color: impl IntoColor)-> Self {
    self.state.clear_color=color.into_color();
    self
  }

  #[inline(always)]
  pub fn with_inner_size(mut self,inner_size: PhysicalSize<u32>)-> Self {
    self.state.inner_size=inner_size;
    self
  }

  pub fn with_scene(mut self,scene: Scene)-> Self {
    self.state.scene=Some(scene);
    self
  }

  #[inline]
  pub fn run(mut self) {
    self.event_loop.run_app(&mut self.state)
    .unwrap();
  }
}

struct State {
  title: String,
  clear_color: Color,
  inner_size: PhysicalSize<u32>,
  renderer: Option<Renderer>,
  window: Option<Arc<Window>>,
  scene: Option<Scene>,
  dragging: bool,
}

impl ApplicationHandler for State {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    if self.window.is_some() {
      return;
    }

    let window=Arc::new(
      event_loop.create_window(
        Window::default_attributes()
        .with_title(mem::take(&mut self.title))
        .with_inner_size(self.inner_size),
      ).expect("failed to create window"),
    );

    let state=pollster::block_on(Renderer::new(window.clone(),self.clear_color));

    self.window=Some(window);
    self.renderer=Some(state);
  }

  fn window_event(&mut self,event_loop: &ActiveEventLoop,window_id: WindowId,event: WindowEvent) {
    let Some(window)=&self.window else {
      return;
    };

    if window.id()!=window_id {
      return;
    }

    match event {
      WindowEvent::CloseRequested=> event_loop.exit(),
      WindowEvent::Resized(size)=> {
        if let Some(renderer)=&mut self.renderer {
          renderer.resize(size);
        }
      },
      WindowEvent::RedrawRequested=> {
        if let Some(renderer)=&mut self.renderer && let Some(scene)=&mut self.scene {
          scene.render(renderer);
        }
      },
      WindowEvent::MouseInput { state, button: MouseButton::Left, .. }=> {
        self.dragging=state==ElementState::Pressed;
      },
      WindowEvent::MouseWheel { delta, .. }=> {
        let amount=match delta {
          MouseScrollDelta::LineDelta(_,y)=> y,
          MouseScrollDelta::PixelDelta(position)=> position.y as f32 / 120.0,
        };

        if let Some(renderer)=&mut self.renderer {
          renderer.zoom_camera(amount);
        }
      },
      WindowEvent::Focused(false)=> self.dragging=false,
      _=> {},
    }
  }

  fn device_event(&mut self,_event_loop: &ActiveEventLoop,_device_id: DeviceId,event: DeviceEvent) {
    if !self.dragging {
      return;
    }

    let DeviceEvent::MouseMotion { delta }=event else {
      return;
    };

    if let Some(renderer)=&mut self.renderer {
      renderer.orbit_camera(delta.0 as f32,delta.1 as f32);
    };
  }

  fn about_to_wait(&mut self,_event_loop: &ActiveEventLoop) {
    if let Some(window)=&self.window {
      window.request_redraw();
    }
  }
}


impl Default for State {
  #[inline(always)]
  fn default()-> Self {
    Self {
      scene: None,
      window: None,
      renderer: None,
      dragging: false,
      title: "brush window".to_owned(),
      clear_color: Color::default(),
      inner_size: PhysicalSize::new(1280,720),
    }
  }
}






