use common::User;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::{
    ConsoleService,
    fetch::{FetchService, FetchTask, Request, Response},
};
use crate::util::utils::{Anchor, AppRoute};
use yew::html;
use yew_router::{components::RouterAnchor, router::Router, Switch};
use serde_derive::Deserialize;

mod user;
mod util;

enum Msg {
    MakeReq,
    Resp(Result<UsersResponse, anyhow::Error>),
}


struct UsersApp {
    link: ComponentLink<Self>,
    users: Option<Vec<User>>,
    fetch_task: Option<FetchTask>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
struct UsersResponse {
    users: Vec<User>
}

impl Component for UsersApp {
    type Message = Msg;
    type Properties = ();
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeReq);
        Self {
            link,
            users: None,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq => {
                self.users = None;
                let req = Request::get("http://localhost:11007/users")
                    .body(Nothing)
                    .expect("can make req to users endpoint");

                let cb = self.link.callback(
                    |response: Response<Json<Result<UsersResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_task = Some(task);
                ()
            }
            Msg::Resp(resp) => {
                if let Ok(data) = resp {
                    self.users = Some(data.users);
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let users = self.users.clone();
        let cb = self.link.callback(|_| Msg::MakeReq);
        let _ = ConsoleService::info(&format!("render Users App: {:?}", users));
        html! {
                   <div class=classes!("user")>
                       <div class=classes!("nav")>
                           <Anchor route=AppRoute::Home>{"Home"}</Anchor>
                       </div>
                       <div class=classes!("content")>
                           <Router<AppRoute, ()>
                               render = Router::render(move |switch: AppRoute| {
                                   match switch {
                                       AppRoute::Detail(user_id) => {
                                           html! {
                                               <div>
                                                    <user::detail::Detail user_id=user_id/>
                                               </div>}
                                       }
                                       AppRoute::Home => {
                                           html! {
                                               <div>
                                                   <div class=classes!("refresh")>
                                                       <button onclick=cb.clone()>
                                                           { "refresh" }
                                                       </button>
                                                   </div>
                                                   <user::list::List users=users.clone()/>
                                               </div>
                                           }
                                       }
                                   }
                               })
                           />
                       </div>
                   </div>
               }
    }
}

fn main() {
    yew::start_app::<UsersApp>();
}
