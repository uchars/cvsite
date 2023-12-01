use leptos::*;
use leptos_use::*;

#[component]
pub fn Cursor() -> impl IntoView {
    let cursors = ["|", ""];
    let (c, set_c) = create_signal(cursors[0]);
    let (interval, _set_interval) = create_signal(500);
    let (idx, set_idx) = create_signal(0);
    use_interval_fn(
        move || {
            set_idx.set((idx.get() + 1) % cursors.len());
            set_c.set(cursors[idx.get()]);
        },
        interval,
    );
    view! {
      <p class="command">{move || c.get()}</p>
    }
}
