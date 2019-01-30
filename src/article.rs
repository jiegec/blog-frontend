use comrak::{markdown_to_html, ComrakOptions};
use stdweb::js;
use stdweb::web::*;
use yew::format::*;
use yew::prelude::*;
use yew::services::fetch::*;
use yew::virtual_dom::VNode;
use serde_derive::Deserialize;
use serde_yaml;
use chrono::prelude::DateTime;

type Context = ();

#[derive(Debug, PartialEq, Deserialize)]
struct FrontMatter {
    layout: String,
    date: String,
    tags: Vec<String>,
    category: String,
    title: String
}

pub struct Model {
    front_matter: Option<FrontMatter>,
    content: String,
    _fetch: FetchService,
    _task: FetchTask,
}

#[derive(Clone, Default, PartialEq)]
pub struct Prop {
    pub article: String,
}

pub enum Msg {
    GotData(String),
    DoNothing,
}

impl Component<Context> for Model {
    // Some details omitted. Explore the examples to get more.

    type Message = Msg;
    type Properties = Prop;

    fn create(prop: Self::Properties, env: &mut Env<Context, Self>) -> Self {
        let request = Request::get(&prop.article).body(Nothing).unwrap();
        let mut fetch = FetchService::new();
        let task = fetch.fetch(
            request,
            env.send_back(|response: Response<Text>| {
                let (_, data) = response.into_parts();
                if let Ok(markdown) = data {
                    Msg::GotData(markdown)
                } else {
                    Msg::DoNothing
                }
            }),
        );
        Model {
            front_matter: None,
            content: String::from("Loading..."),
            _fetch: fetch,
            _task: task,
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::GotData(string) => {
                let index = (&string[2..].find("---").unwrap()) + 1;
                self.front_matter = Some(serde_yaml::from_str(&string[..index]).unwrap());
                self.content = String::from(&string[index+6..]);
                true
            }
            Msg::DoNothing => false,
        }
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        if let Some(front_matter) = &self.front_matter {
            let content = markdown_to_html(&self.content, &ComrakOptions::default());
            let node = Node::from_html(&format!("<div>{}</div>", content)).unwrap();
            js! {
                setTimeout(() => {
                    Prism.highlightAll();
                });
            };
            html! {
                <article class="post",>
                    <header class="post-header",>
                        <h1 class="post-title",>
                            { &front_matter.title }
                        </h1>
                        <p class="post-meta",>
                            <time datetime=&front_matter.date,>
                                {
                                    DateTime::parse_from_str(&front_matter.date, "%Y-%m-%d %H:%M:%S %z").unwrap()
                                }
                            </time>
                            { "Tags:" }
                            { for {
                                    front_matter.tags.iter().map(|tag| {
                                        let url = format!("/tag/{}", tag);
                                        html! {
                                            <a href=url,>
                                                { tag }
                                            </a>
                                        }
                                    })
                                }
                            }
                        </p>
                    </header>
                    <div class="post-content",>
                        { VNode::VRef(node) }
                    </div>
                </article>
            }
        } else {
            html! {
                <div>
                    { &self.content }
                </div>
            }
        }
    }
}
