use crate::models::fetchitem::{FetchItem, FetchValueType};
use leptos::*;
use leptos_use::*;

#[component]
fn Fetchlist(items: Vec<FetchItem>) -> impl IntoView {
    view! {
      <p>Name: Nils Sterz</p>
    }
}

#[component]
pub fn Neofetch() -> impl IntoView {
    let cursors = ["|", ""];
    let (c, set_c) = create_signal(cursors[0]);
    let (interval, set_interval) = create_signal(500);
    let (idx, set_idx) = create_signal(0);

    use_interval_fn(
        move || {
            set_idx.set((idx.get() + 1) % cursors.len());
            set_c.set(cursors[idx.get()]);
        },
        interval,
    );

    let fetchItems = vec![
        FetchItem {
            name: String::from("Name"),
            value: String::from("Nils Sterz"),
            value_type: FetchValueType::Text,
        },
        FetchItem {
            name: String::from("Uptime"),
            value: String::from("8813 days, 12 hours, 11 minutes"),
            value_type: FetchValueType::Text,
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
        <div class="terminal-input">
          <p class="ps1">"[nils@cvsite:~/]$"</p><p class="command">{move || c.get()}</p>
        </div>
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
            FetchValueType::Link(url)=> {view!{<a class="fetch-link" target="_blank" href=url.clone()>{ item.value.clone() }</a>}},
            FetchValueType::Text => {view!{<a>{ item.value.clone() }</a>}},
          }
      }
      </div>
      </div>
    }
}
