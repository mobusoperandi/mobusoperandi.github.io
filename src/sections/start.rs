use super::Section;
use maud::html;
pub fn section() -> Section {
    Section::new(
        "start".into(),
        "flex flex-col justify-items-center".into(),
        None,
        html! {
            h1 class="[word-spacing:100vw] text-center text-[min(7vw,15vh)] [line-height:1]" { "STUDY RUST ONLINE IN MOB-PROGRAMMING FORMAT" }
            a class="border border-r-4 p-4 text-center" { "SEE EXISTING MOBS" }
        },
    )
}
