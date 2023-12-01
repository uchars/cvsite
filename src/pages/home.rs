use crate::components::neofetch::Neofetch;
use crate::components::skill::SkillElement;
use crate::models::skill::Skill;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    let skills = vec![
        Skill {
            name: String::from("C++"),
            percentage: 80,
            notes: String::from("Hello"),
        },
        Skill {
            name: String::from("Python"),
            percentage: 60,
            notes: String::from("Pog"),
        },
    ];
    view! {
      <Neofetch/>
      // <div>
      //   {skills.iter().map(|s| {
      //   view! {<SkillElement skill=s.clone()/>}
      // }).collect::<Vec<_>>()}
      // </div>
    }
}
