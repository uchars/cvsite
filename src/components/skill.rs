use crate::models::skill::Skill;
use leptos::*;

#[component]
pub fn PercentageBar(color: String, percentage: u8, background: String) -> impl IntoView {
    view! {
      <div class="progress-bar" style=format!("
        --progress-percentage: {percentage}%;
        --progress-background: {background}
      ")>
      <span class="progress-indicator" style=format!("--progress-color: {color};--progress-color-end: purple")>{percentage} "%"</span>
      </div>
    }
}

#[component]
pub fn SkillElement(skill: Skill) -> impl IntoView {
    view! {
      <div class="progress-container">
      <h3>{skill.name}</h3>
      <PercentageBar color=String::from("#722F37") percentage=skill.percentage background=String::from("#E6E6FA")/>
      </div>
    }
}
