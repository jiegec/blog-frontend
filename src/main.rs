#[macro_use]
extern crate yew;
#[macro_use]
extern crate stdweb;
extern crate serde_derive;
extern crate serde_yaml;
extern crate comrak;
extern crate regex;

mod article;

use stdweb::web::*;
use yew::prelude::*;

type Context = ();

struct Model {
    article: Option<String>,
}

enum Msg {}

impl Component<Context> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
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

    fn update(&mut self, _: Self::Message, _: &mut Env<Context, Self>) -> ShouldRender {
        false
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
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
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
