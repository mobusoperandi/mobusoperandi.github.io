use super::Section;
use maud::html;
pub fn section() -> Section {
    Section::new(
        "main".into(),
        "flex flex-col items-center justify-center".into(),
        None,
        html! {
            h1 class="[word-spacing:min(100vw,100vh)] text-center text-[min(7vw,11vh)] [line-height:1.1]" {
                p {"STUDY RUST ONLINE"}
                p {"IN"}
                p {"MOB-PROGRAMMING FORMAT"}
            }
            a href="#mobs_calendar" class="border-4 rounded-xl p-4 text-center border-current text-xl mt-10" { "SEE EXISTING MOBS" }
        },
    )
}
