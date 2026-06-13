// air_client2/src/dispatcher/wayland/handlers/registry.rs
use crate::dispatcher::wayland::state::WaylandState;
use wayland_client::{
    protocol::{
        wl_compositor::WlCompositor, wl_output::WlOutput, wl_registry::WlRegistry, wl_seat::WlSeat,
        wl_shm::WlShm,
    },
    Connection, Dispatch, QueueHandle,
};

impl Dispatch<WlRegistry, ()> for WaylandState {
    fn event(
        state: &mut Self,
        registry: &WlRegistry,
        event: wayland_client::protocol::wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wayland_client::protocol::wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            match interface.as_str() {
                "wl_compositor" => {
                    let compositor = registry.bind::<WlCompositor, _, _>(name, version, qh, ());
                    state.compositor = Some(compositor);
                    state.create_surface(qh);
                    println!("✅ Compositor registered");
                }
                "wl_shm" => {
                    let shm = registry.bind::<WlShm, _, _>(name, version, qh, ());
                    state.shm = Some(shm);
                    println!("✅ SHM registered");
                }
                "wl_seat" => {
                    let seat = registry.bind::<WlSeat, _, _>(name, version, qh, ());
                    state.seat = Some(seat);
                    println!("✅ Seat registered");
                }
                "wl_output" => {
                    let output = registry.bind::<WlOutput, _, _>(name, version, qh, ());
                    state.outputs.push(output);
                    println!("📺 WlOutput registered: id={}", name);
                }
                "xdg_wm_base" => {
                    let wm_base = registry.bind::<wayland_protocols::xdg::shell::client::xdg_wm_base::XdgWmBase, _, _>(name, version, qh, ());
                    state.wm_base = Some(wm_base);
                    println!("✅ XDG WM Base registered");
                }
                _ => {}
            }
        }
    }
}
