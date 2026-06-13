// air_client2/src/dispatcher/wayland/state.rs
use crate::HandlerCommand;
use std::{collections::HashMap, os::fd::AsFd};
use wayland_client::{
    backend::ObjectId,
    delegate_noop,
    protocol::{
        wl_buffer::WlBuffer,
        wl_compositor::WlCompositor,
        wl_keyboard::WlKeyboard,
        wl_output::WlOutput,
        wl_pointer::WlPointer,
        wl_seat::WlSeat,
        wl_shm::{self, WlShm},
        wl_shm_pool::WlShmPool,
        wl_surface::WlSurface,
    },
    QueueHandle,
};
use wayland_protocols::xdg::shell::client::{xdg_surface::XdgSurface, xdg_wm_base::XdgWmBase};

delegate_noop!(WaylandState: ignore WlCompositor);
delegate_noop!(WaylandState: ignore WlSurface);
delegate_noop!(WaylandState: ignore WlShm);
delegate_noop!(WaylandState: ignore WlShmPool);
delegate_noop!(WaylandState: ignore WlBuffer);

pub struct WaylandState {
    pub command_tx: flume::Sender<HandlerCommand>,
    pub registry: Option<wayland_client::protocol::wl_registry::WlRegistry>,
    pub compositor: Option<WlCompositor>,
    pub seat: Option<WlSeat>,
    pub pointer: Option<WlPointer>,
    pub keyboard: Option<WlKeyboard>,
    pub wm_base: Option<XdgWmBase>,
    pub shm: Option<WlShm>,
    pub surface: Option<WlSurface>,
    pub xdg_surface: Option<XdgSurface>,
    pub buffer: Option<WlBuffer>,
    pub virtual_output_name: String,
    pub is_on_virtual: bool,

    pub outputs: Vec<WlOutput>,
    pub output_names: HashMap<ObjectId, String>,
    pub virtual_output_id: Option<ObjectId>,
    #[allow(unused)]
    pub current_output_id: Option<ObjectId>,
}

impl WaylandState {
    pub fn new(command_tx: flume::Sender<HandlerCommand>, virtual_output_name: String) -> Self {
        Self {
            command_tx,
            registry: None,
            compositor: None,
            seat: None,
            pointer: None,
            keyboard: None,
            wm_base: None,
            shm: None,
            surface: None,
            xdg_surface: None,
            buffer: None,
            virtual_output_name,
            is_on_virtual: false,
            outputs: Vec::new(),
            output_names: HashMap::new(),
            virtual_output_id: None,
            current_output_id: None,
        }
    }

    pub fn create_surface(&mut self, qh: &QueueHandle<Self>) {
        if let Some(compositor) = &self.compositor {
            let surface = compositor.create_surface(qh, ());
            self.surface = Some(surface);
            println!("✅ Surface created");
        }
    }

    pub fn create_fullscreen_window(&mut self, qh: &QueueHandle<Self>, output: &WlOutput) {
        if let (Some(wm_base), Some(surface)) = (&self.wm_base, &self.surface) {
            // НЕ создаём если уже есть
            if self.xdg_surface.is_some() {
                return;
            }

            let xdg_surface = wm_base.get_xdg_surface(surface, qh, ());
            let toplevel = xdg_surface.get_toplevel(qh, ());
            toplevel.set_title("Air Client".into());
            toplevel.set_fullscreen(Some(output));
            toplevel.set_min_size(1, 1);
            surface.commit();

            self.xdg_surface = Some(xdg_surface);
            println!("✅ Fullscreen window created on virtual output");
        }
    }

    pub fn create_buffer(&mut self, qh: &QueueHandle<Self>, width: i32, height: i32) {
        if let (Some(shm), Some(surface)) = (&self.shm, &self.surface) {
            if self.buffer.is_some() {
                return;
            }

            let stride = width * 4;
            let size = (stride * height) as u64;

            let mut file = tempfile::tempfile().unwrap();
            file.set_len(size).unwrap();
            let black = vec![0u8; size as usize];
            std::io::Write::write_all(&mut file, &black).unwrap();

            let pool = shm.create_pool(file.as_fd(), size as i32, qh, ());
            let buffer =
                pool.create_buffer(0, width, height, stride, wl_shm::Format::Argb8888, qh, ());

            surface.attach(Some(&buffer), 0, 0);
            surface.commit();

            self.buffer = Some(buffer);
            println!("✅ Buffer created: {}x{}", width, height);
        }
    }
}
