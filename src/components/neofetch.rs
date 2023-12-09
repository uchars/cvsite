use crate::components::messages::Messages;
use crate::models::fetchitem::{FetchItem, FetchValueType};
use crate::utils::get_time;
use leptos::*;

#[component]
fn Fetchlist(items: Vec<FetchItem>) -> impl IntoView {
    view! {
      <div class="fetch-text-container">
        {items.iter().map(|i| {
          view! {<NeofetchLine item=i.clone()/>}
        }).collect::<Vec<_>>()}
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
            value: String::from("wezterm + tmux"),
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
        FetchItem {
            name: String::from("Contact"),
            value: String::from("email@nils.sterz"),
            value_type: FetchValueType::EMail(String::from("email@nils.sterz")),
        },
    ];

    view! {
      <div class="terminal-input">
        <p class="ps1">"[nils@cvsite:~/]$"</p><p class="command">fetch</p>
      </div>
      <div class="neofetch-container">
        <div class="neofetch-split">
          <img class="fetch-image" alt="nils" src="public/images/nils.webp"/>
          <Fetchlist items={fetchItems}/>
        </div>
      </div>
      <Messages />
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
            FetchValueType::Text => {view!{<div>{ item.value.clone() }</div>}},
            FetchValueType::HoverText(hover) => {view!{<div class="fetch-hover" title={hover}>{item.value.clone()}</div>}},
            FetchValueType::EMail(mail) => {view!{<div><a class="fetch-link" target="_blank" href=mail.clone()>{ item.value.clone() }</a></div>}}
          }
      }
      </div>
      </div>
    }
}
