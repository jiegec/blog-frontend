extern crate comrak;
extern crate regex;
extern crate serde_derive;
extern crate serde_yaml;

mod article;

use stdweb::web::*;
use yew::{html, html_impl, prelude::*};

struct Model {
    article: Option<String>,
}

enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let path_re =
            regex::Regex::new(r"^/([a-z]+)/(\d{4})/(\d{2})/(\d{2})/([a-z0-9-]+)/?$").unwrap();
        let location = window().location().unwrap();
        let path = location.pathname().unwrap();

        if let Some(cap) = path_re.captures(&path) {
            let url = format!(
                "https://raw.githubusercontent.com/jiegec/blog-source/master/_posts/{}-{}-{}-{}.md",
                &cap[2], &cap[3], &cap[4], &cap[5]
            );
            Model { article: Some(url) }
        } else {
            Model { article: None }
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        if let Some(url) = &self.article {
            html! {
                <div>
                    <article::Model: article=url.clone(),/>
                </div>
            }
        } else {
            html! {
                <div>
                    { "404 Not Found" }
                </div>
            }
        }
    }
}

fn main() {
    yew::initialize();
    let app: App<Model> = App::new();
    app.mount_to_body();
    yew::run_loop();
}
