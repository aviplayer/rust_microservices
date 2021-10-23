use common::User;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub user_id: i32,
}

pub struct Detail {
    props: Props,
    link: ComponentLink<Self>,
    user: Option<User>,
    fetch_task: Option<FetchTask>,
}

impl Detail {
    fn render_detail(&self, user: &Option<User>) -> Html {
        match user {
            Some(u) => {
                let admin = if u.site_admin {
                    Some("admin")
                } else {
                    Some("")
                };

                return html! {
                    <div class=classes!("detail")>
                        <h1>{&u.login}{" ("}<span class=classes!("id")>{u.id}</span>{")"}</h1>
                        <div>{"Node id: "}{&u.node_id}</div>
                        <div><img src={u.avatar_url.clone()}/></div>
                        <div class=classes!(admin)>{if u.site_admin { "Admin" } else { "not admin" }}</div>
                    </div>
                };
            }
            None => {
                return html! {
                    <div class=classes!("loading")>{"loading..."}</div>
                };
            }
        }
    }
}

pub enum Msg {
    MakeReq(i32),
    Resp(Result<User, anyhow::Error>),
}

impl Component for Detail {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeReq(props.user_id));
        Self {
            props,
            link,
            user: None,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq(id) => {
                let req = Request::get(&format!(
                    "http://localhost:11007/user/{}",
                    id
                ))
                    .body(Nothing)
                    .expect("can make req to users api");

                let cb =
                    self.link
                        .callback(|response: Response<Json<Result<User, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::Resp(data)
                        });

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_task = Some(task);
                ()
            }
            Msg::Resp(resp) => {
                if let Ok(data) = resp {
                    self.user = Some(data);
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        return html! {
            <div>
                { self.render_detail(&self.user)}
            </div>
        };
    }
}
