use leptos::*;

use crate::models::project::Project;

#[component]
fn ProjectItem(p: Project) -> impl IntoView {
    view! {
    <div class="output">-rw-r--r-- 1 nils users {p.name.clone()} -> <a target="_blank" class="project-link" href=p.url.clone()>{p.url.clone()}</a></div>
      }
}
#[component]
pub fn Infosection() -> impl IntoView {
    let projects = vec![
        Project::new("nixvim", "green", "https://github.com/uchars/nixvim"),
        Project::new(
            "nixos-config",
            "green",
            "https://github.com/uchars/nixos-config",
        ),
        Project::new("cvsite", "green", "https://github.com/uchars/cvsite"),
    ];
    view! {
      <div class="terminal-input">
        <p class="ps1" style="margin:0;padding:0;padding-right:5px">"[nils@cvsite:~/projects]$ " </p><p class="command" style="margin:0;padding:0">ls -l</p>
      </div>
      <div class="output">total {projects.len()}</div>
      {projects.iter().map(|p| { view! {<ProjectItem p=p.clone()/>}}).collect::<Vec<_>>()}
    }
}
