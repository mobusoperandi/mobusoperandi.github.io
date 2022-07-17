use super::Section;
use maud::html;
pub fn section() -> Section {
    Section::new(
        "start".into(),
        "".into(),
        None,
        html! {
            h1 class="[word-spacing:100vw] text-center text-[min(7vw,15vh)] [line-height:1]" { "STUDY RUST ONLINE IN MOB-PROGRAMMING FORMAT" }
        },
    )

    // "main".into(),
    // "".into(),
    // None,
    // html! {
    //     h1 { "An online community for the regular study of software development in mob programming format" }
    //     ul {
    //         li { a href="#in_the_media" { "in_the_media" } }
    //         li { a href="#mobs_calendar" { "mobs_calendar" } }
    //         li { a href="#join" { "join" } }
    //         li { a href="#mightyiam" { "mightyiam" } }
    //         li { a href="#why_mob" { "why_mob" } }
    //     }
    // },
}
