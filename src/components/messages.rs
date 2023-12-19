use crate::components::cursor::Cursor;
use leptos::*;
use std::time::Duration;

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
    let (index, set_index) = create_signal(0);
    let (is_writing, set_writing) = create_signal(true);

    set_interval(
        move || {
            if !is_writing.get() {
                return;
            }
            if word_part.get().len() < messages[index.get()].len() {
                let mut w = word_part.get().clone();
                w.push(messages[index.get()].as_bytes()[w.len()] as char);
                set_part(w);
            } else {
                set_writing(false);
                set_timeout(
                    move || {
                        set_part(String::from(""));
                        set_index((index.get() + 1) % messages.len());
                        set_writing(true);
                    },
                    Duration::from_secs(3),
                );
            }
        },
        Duration::from_millis(85),
    );

    view! {
      <div class="terminal-input">
        <p class="ps1">"[nils@cvsite:~/]$ " </p><p class="command">{move || word_part.get()}</p><Cursor writing_sig=is_writing/>
      </div>
    }
}
