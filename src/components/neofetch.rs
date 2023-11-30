use crate::models::fetchitem::FetchItem;
use leptos::*;

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

    view! {
      <div class="neofetch-container">
      <p>"[nils@cvsite:~/]$ fetch"</p>
      <div class="neofetch-split">
      <image class="fetch-image" src="/assets/images/nils.jpg"/>
      // <div class="fetch-text">
      //   Hello
      // </div>
      <div class="fetch-text">
        <p>Name: Nils Sterz</p>
        <p>Uptime: 8813 days, 12 hours, 11 minutes</p>
        <p>Country: Germany "🇩🇪"</p>
        <p>Memory: 0 / 15928MiB</p>
        <p>OS: NixOS 23.05.20231104.78f3a4a (Stoat) x86_64</p>
        <p>Terminal: tmux</p>
        <p>Editor: neovim</p>
      </div>
      </div>
      <p>"[nils@cvsite:~/]$ "{move || c.get()}</p>
      </div>
    }
}
