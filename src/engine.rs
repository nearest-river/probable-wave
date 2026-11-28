
use glam::prelude::*;
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

  #[inline(always)]
  pub fn with_scene(mut self,scene: Scene)-> Self {
    self.state.scene=scene;
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
  scene: Scene,
  dragging: bool,

  egui_ctx: egui::Context,
  egui_state: Option<egui_winit::State>,
  egui_renderer: Option<egui_wgpu::Renderer>,

  clip: Vec3,
  intensity: f32,

  n: u32,
  l: u32,
  m: i32
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

    let renderer=pollster::block_on(Renderer::new(window.clone(),self.clear_color));
    let egui_state=egui_winit::State::new(
      self.egui_ctx.clone(),
      egui::ViewportId::ROOT,
      &window,
      Some(window.scale_factor() as f32),
      None,
      None,
    );

    let egui_renderer=egui_wgpu::Renderer::new(
      &renderer.dev,
      renderer.surface_format(),
      egui_wgpu::RendererOptions::PREDICTABLE,
    );

    self.egui_state=Some(egui_state);
    self.egui_renderer=Some(egui_renderer);

    self.window=Some(window);
    self.renderer=Some(renderer);
  }

  fn window_event(&mut self,event_loop: &ActiveEventLoop,window_id: WindowId,event: WindowEvent) {
    let Some(window)=self.window.clone() else {
      return;
    };

    if window.id()!=window_id {
      return;
    }

    if let Some(egui_state)=&mut self.egui_state {
      let response=egui_state.on_window_event(&window, &event);

      if response.consumed {
        self.dragging=false;
      }
    }

    match event {
      WindowEvent::CloseRequested=> event_loop.exit(),
      WindowEvent::Resized(size)=> {
        if let Some(renderer)=&mut self.renderer {
          renderer.resize(size);
        }
      },
      WindowEvent::RedrawRequested=> {
        let Some(egui_state)=&mut self.egui_state else {
          return;
        };

        let raw_input=egui_state.take_egui_input(&window);
        let ctx=self.egui_ctx.clone();

        let mut quantum_state_changed=false;
        let output=ctx.run_ui(raw_input, |_| {
          quantum_state_changed=self.control_panel();
        });

        let egui::FullOutput { platform_output, textures_delta, shapes, pixels_per_point, .. }=output;

        self.egui_state
        .as_mut()
        .unwrap()
        .handle_platform_output(&window,platform_output);

        let paint_jobs=ctx.tessellate(shapes, pixels_per_point);

        if quantum_state_changed {
          let Some(renderer)=&self.renderer else {
            return;
          };

          self.scene.set_quantum_state(
            QuantumState::new(
              self.n,
              self.l,
              self.m,
              SpinProjection::Up,
            )
            .unwrap(),
            renderer,
          );
        }

        let (Some(renderer), Some(egui_renderer))=(&mut self.renderer, &mut self.egui_renderer) else {
          return;
        };

        self.scene.render(
          renderer,
          self.clip.to_array(),
          self.intensity,
          egui_renderer,
          &paint_jobs,
          &textures_delta
          );
      },
      WindowEvent::MouseInput { state, button: MouseButton::Left, .. }=> {
        self.dragging=state == ElementState::Pressed;
      },
      WindowEvent::MouseWheel { delta, .. }=> {
        let amount=match delta {
          MouseScrollDelta::LineDelta(_, y)=> y,
          MouseScrollDelta::PixelDelta(position)=> {
            position.y as f32 / 120.0
          },
        };

        if let Some(renderer)=&mut self.renderer {
          renderer.zoom_camera(amount);
        }
      },
      WindowEvent::Focused(false)=> {
        self.dragging=false;
      },
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
      renderer.orbit_camera(delta.0 as f32,-delta.1 as f32);
    };
  }

  fn about_to_wait(&mut self,_event_loop: &ActiveEventLoop) {
    if let Some(window)=&self.window {
      window.request_redraw();
    }
  }
}

impl State {
  fn control_panel(&mut self)-> bool {
    use egui::*;
    let mut changed=false;

    egui::Window::new("hydrogen atom")
    .default_pos([20.0, 20.0])
    .default_width(260.0)
    .resizable(false)
    .show(&self.egui_ctx, |ui| {
      ui.heading("quantum state");
      ui.add_space(8.0);

      changed|=ui.add(Slider::new(&mut self.n, 1..=7).text("n · principal"))
      .changed();

      self.l=self.l.min(self.n - 1);

      changed|=ui.add(Slider::new(&mut self.l, 0..=self.n - 1).text("l · azimuthal"))
      .changed();

      self.m=self.m.clamp(-(self.l as i32), self.l as i32);

      changed|=ui.add(Slider::new(&mut self.m,-(self.l as i32)..=(self.l as i32)).text("m · magnetic"))
      .changed();

      ui.add_space(12.0);
      ui.separator();
    });

    changed
}
}


impl Default for State {
  #[inline(always)]
  fn default()-> Self {
    Self {
      window: None,
      renderer: None,
      dragging: false,
      scene: Scene::new(),
      title: "brush window".to_owned(),
      clear_color: Color::default(),
      inner_size: PhysicalSize::new(1280,720),

      egui_ctx: egui::Context::default(),
      egui_state: None,
      egui_renderer: None,

      clip: Vec3::ONE,
      intensity: 1.0,
      n: 4,
      l: 3,
      m: 1,
    }
  }
}






