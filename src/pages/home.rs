use crate::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

pub struct Person {
    name: String,
    age: u8,
    languages: Vec<String>,
    operating_system: String,
}

#[function_component(Home)]
pub fn home() -> Html {
    let person = Person {
        name: "MunJitso".to_string(),
        age: 17,
        languages: vec![
            "Rust".to_string(),
            "Java".to_string(),
            "Python".to_string(),
            "Javascript".to_string(),
        ],
        operating_system: "Windows".to_string(),
    };
    let mut langs = String::new();
    if person.languages.is_empty() {
        langs = "Empty".to_string();
    } else {
        for n in person.languages.iter() {
            if n == person.languages.get(0).unwrap() {
                langs = n.to_string();
            } else {
                langs = format!("{} - {}", langs, n);
            }
        }
    }

    html!(
        <div class="container">
                <div class="sub-container">
                    <div class="card-title">
                        <h1>{" Infos: "}</h1>
                    </div>
                    <div class="card-infos">
                        <p>{"Name : "}{person.name}</p>
                        <p>{"Age : "}{person.age}</p>
                        <p>{"Language : "}{langs}</p>
                        <p>{"OS : "}{person.operating_system}</p>
                        <Link<Route> to={Route::Repos}>{ "click here to go home" }</Link<Route>>
                    </div>
                </div>
                <div class="sub-container">
                    <div class="card-title">
                        <h1>{" About: "}</h1>
                    </div>
                    <div class="card-infos">
                        <p>{"I started exploring the world of computers since I was a kiddo, I was doing some basic stuff like
                            typing in Microsoft Word, making animated presentations in Microsoft Powerpoint, ediiting Blogger's
                            templates... etc. At the age of 15, I began learning programming languages and sadly it was a
                            dynamically typed language (Javascript), then I kept exploring more. Currently, I'm coding Rust, and
                            I also have some intentions to grow my knowledge more about Networking, Cloud and Operating
                            systems."}</p>
                    </div>
                </div>
            </div>
    )
}
