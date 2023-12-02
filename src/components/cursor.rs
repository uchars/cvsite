use leptos::*;
use std::time::Duration;

#[component]
pub fn Cursor() -> impl IntoView {
    let cursors = ["|", ""];
    let (c, set_c) = create_signal(cursors[0]);
    let (idx, set_idx) = create_signal(0);
    set_interval(
        move || {
            set_idx.set((idx.get() + 1) % cursors.len());
            set_c.set(cursors[idx.get()]);
        },
        Duration::from_millis(500),
    );
    view! {
      <p class="command">{move || c.get()}</p>
    }
}
