use leptos::*;

pub fn TerminalPane(content: impl IntoView) -> impl IntoView {
    view! {
      <div class="terminal-pane">
        { content }
      </div>
    }
}
