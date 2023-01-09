use reqwasm::http::Request;
use yew::prelude::*;

#[function_component(Repos)]
fn repos() -> Html {
  wasm_bindgen_futures::spawn_local(async move {
    let resp = Request::get("https://api.github.com/users/munjitso/repos?sort=updated").send().await.unwrap().text();
    match resp {
      Ok(output) => println!("{}", output),
      Err(e) => println!("{}", e)
    }
    println!("{}", resp)
  });
  html!(
      <p>{"Hello"}</p>
  )
}
