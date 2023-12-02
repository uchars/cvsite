use leptos::*;
use std::time::Duration;

#[component]
pub fn Cursor(writing_sig: ReadSignal<bool>) -> impl IntoView {
    let cursors = ["|", ""];
    let (c, set_c) = create_signal(cursors[0]);
    let (idx, set_idx) = create_signal(0);
    set_interval(
        move || {
            if writing_sig.get() {
                set_c.set(cursors[0]);
                return;
            }
            set_idx.set((idx.get() + 1) % cursors.len());
            set_c.set(cursors[idx.get()]);
        },
        Duration::from_millis(500),
    );
    view! {
      <p class="command">{move || c.get() }</p>
    }
}
