use crate::models::fetchitem::{FetchItem, FetchValueType};
use chrono::{Local, NaiveDate, NaiveDateTime, Timelike};
use leptos::{
    leptos_dom::logging::{console_error, console_log},
    *,
};
use leptos_use::*;
use web_sys::console::info;

#[component]
fn Fetchlist(items: Vec<FetchItem>) -> impl IntoView {
    view! {
      <p>Name: Nils Sterz</p>
    }
}

fn get_time() -> String {
    let target_date = NaiveDate::from_ymd(1999, 10, 14).and_hms(0, 0, 0);

    // Get the current date and time in local time
    let current_date = Local::now();

    // Convert current date to NaiveDateTime
    let current_date_naive = NaiveDateTime::from_timestamp(current_date.timestamp(), 0);

    // Calculate the duration between the target date and the current date
    let duration = current_date_naive.signed_duration_since(target_date);

    // Extract days, hours, and minutes from the duration
    let days = duration.num_days();
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;

    // Format the result as a string
    format!("{} days, {} hours, {} minutes", days, hours, minutes)
}

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

#[component]
fn Messages() -> impl IntoView {
    let messages = [
        "Vim enthusiast",
        "Espressif developer",
        "C++ developer",
        "Rust developer",
        "Hessian",
    ];
    let (word, set_word) = create_signal(messages[0]);
    let (word_part, set_part) = create_signal(String::from(""));
    let (interval, set_interval) = create_signal(250_u64);
    let (index, set_index) = create_signal(0);

    use_interval_fn(
        move || {
            console_log("Heyo");
            if word_part.get().len() < messages[index.get()].len() {
                let mut w = word_part.get().clone();
                w.push(messages[index.get()].as_bytes()[w.len()] as char);
                set_part(w);
            } else {
                set_part(String::from(""));
                set_index((index.get() + 1) % messages.len());
                set_word(messages[index.get()]);
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

#[component]
pub fn Neofetch() -> impl IntoView {
    let fetchItems = vec![
        FetchItem {
            name: String::from("Name"),
            value: String::from("Nils Sterz"),
            value_type: FetchValueType::Text,
        },
        FetchItem {
            name: String::from("Uptime"),
            value: get_time(),
            value_type: FetchValueType::HoverText(String::from("14. October 1999")),
        },
        FetchItem {
            name: String::from("Country"),
            value: String::from("Germany 🇩🇪"),
            value_type: FetchValueType::Text,
        },
        FetchItem {
            name: String::from("Memory"),
            value: String::from("0MiB / 15928MiB"),
            value_type: FetchValueType::Text,
        },
        FetchItem {
            name: String::from("OS"),
            value: String::from("NixOS 23.05.20231104.78f3a4a (Stoat) x86_64"),
            value_type: FetchValueType::Text,
        },
        FetchItem {
            name: String::from("Terminal"),
            value: String::from("tmux"),
            value_type: FetchValueType::Text,
        },
        FetchItem {
            name: String::from("Editor"),
            value: String::from("neovim"),
            value_type: FetchValueType::Text,
        },
        FetchItem {
            name: String::from("GitHub"),
            value: String::from("github.com/uchars"),
            value_type: FetchValueType::Link(String::from("https://github.com/uchars/")),
        },
        FetchItem {
            name: String::from("LinkedIn"),
            value: String::from("linkedin.com/in/nils-sterz"),
            value_type: FetchValueType::Link(String::from("https://linkedin.com/in/nils-sterz")),
        },
    ];

    view! {
      <div class="neofetch-container">
        <div class="terminal-input">
          <p class="ps1">"[nils@cvsite:~/]$"</p><p>"fetch"</p>
        </div>
        <div class="neofetch-split">
          <img class="fetch-image" src="img/nils.jpg"/>
          <div class="fetch-text-container">
            {fetchItems.iter().map(|i| {
          view! {<NeofetchLine item=i.clone()/>}
        }).collect::<Vec<_>>()}
          </div>
        </div>
        <Messages />
      </div>
    }
}

#[component]
fn NeofetchLine(item: FetchItem) -> impl IntoView {
    view! {
      <div class="fetch-text">
      <div class="fetch-text-title">
      {item.name}:
      </div>
      <div class="fetch-text-value">
      {
          match item.value_type {
            FetchValueType::Link(url)=> {view!{<div><a class="fetch-link" target="_blank" href=url.clone()>{ item.value.clone() }</a></div>}},
            FetchValueType::Text => {view!{<div><a>{ item.value.clone() }</a></div>}},
            FetchValueType::HoverText(hover) => {view!{<div class="fetch-hover" title={hover}>{item.value.clone()}</div>}},
          }
      }
      </div>
      </div>
    }
}
