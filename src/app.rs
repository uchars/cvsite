use crate::pages::home::Home;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
      <Title text="Nils Sterz"/>
        <Router>
        <Routes>
        <Route path="/*any" view=Home/>
        </Routes>
      </Router>
    }
}
