use crate::components::infosection::Infosection;
use crate::components::neofetch::Neofetch;
use crate::components::terminal_pane::TerminalPane;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
      <div class="terminal">
        {TerminalPane(Neofetch())}
        {TerminalPane(Infosection())}
      </div>
    }
}
