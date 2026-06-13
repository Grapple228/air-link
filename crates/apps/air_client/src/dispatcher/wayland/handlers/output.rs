// air_client2/src/dispatcher/wayland/handlers/output.rs
use crate::{config, dispatcher::wayland::state::WaylandState};
use wayland_client::{
    protocol::wl_output::{Event, WlOutput},
    Connection, Dispatch, Proxy, QueueHandle,
};

impl Dispatch<WlOutput, ()> for WaylandState {
    fn event(
        state: &mut Self,
        output: &WlOutput,
        event: Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            Event::Name { name } => {
                let output_id = output.id();
                println!("📺 Output {} name: '{}'", output_id, name);
                state.output_names.insert(output_id.clone(), name.clone());

                if name == state.virtual_output_name {
                    println!(
                        "🎯 Virtual output identified: {} (id={:?})",
                        name, output_id
                    );
                    state.virtual_output_id = Some(output_id);

                    // создаём окно
                    state.create_fullscreen_window(qh, output);
                    state.create_buffer(qh, config().WIDTH as i32, config().HEIGHT as i32);
                }
            }
            _ => {}
        }
    }
}
