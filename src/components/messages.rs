use crate::components::cursor::Cursor;
use leptos::*;
use leptos_use::*;

#[component]
pub fn Messages() -> impl IntoView {
    let messages = [
        "Vim enthusiast",
        "Espressif developer",
        "C++ developer",
        "Rust developer",
        "Hessian",
    ];
    let (word_part, set_part) = create_signal(String::from(""));
    let (interval, set_interval) = create_signal(250_u64);
    let (index, set_index) = create_signal(0);

    use_interval_fn(
        move || {
            if word_part.get().len() < messages[index.get()].len() {
                let mut w = word_part.get().clone();
                w.push(messages[index.get()].as_bytes()[w.len()] as char);
                set_part(w);
            } else {
                set_part(String::from(""));
                set_index((index.get() + 1) % messages.len());
            }
        },
        interval,
    );

    view! {
      <div class="terminal-input">
        <p class="ps1">"[nils@cvsite:~/]$ " </p>{move || word_part.get()}<Cursor />
      </div>
    }
}
