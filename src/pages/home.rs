use crate::components::picture::Headshot;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
      <h1>"Nils Sterz"</h1>
      <Headshot/>
    }
}
