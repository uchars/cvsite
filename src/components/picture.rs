use leptos::*;

#[component]
pub fn Headshot() -> impl IntoView {
    view! {
      <div class="image-container">
      <image width="300" height="300" src="/assets/images/nils.jpg"/>
      </div>
    }
}
