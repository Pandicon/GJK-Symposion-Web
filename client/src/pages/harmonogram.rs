use yew::prelude::*;

const VALID_DAYS: [&str; 3] = ["streda", "ctvrtek", "patek"];

#[derive(PartialEq, Properties, Debug)]
pub struct Props {
    pub day: Option<String>
}

#[function_component(Harmonogram)]
pub fn harmonogram(props: &Props) -> Html {
    let day = if let Some(day) = &props.day {
        let day_lowercase = day.to_ascii_lowercase();
        if !VALID_DAYS.contains(&day_lowercase.as_str()) {
            String::from("all")
        } else {
            day_lowercase
        }
    } else {
        String::from("all")
    };
    html! {
        <>
        <h1>{"Nazdárek!"}</h1>
        <h2>{"Zde je harmonogram :D"}</h2>
        <p>{"Den: "}{day}</p>
        </>
    }
}