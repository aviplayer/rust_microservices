use common::User;
use yew::prelude::*;

use crate::util::utils::{Anchor, AppRoute};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub users: Option<Vec<User>>,
}

pub struct List {
    props: Props,
}

impl List {
    pub fn render_list(&self, users: &Option<Vec<User>>) -> Html {
        if let Some(u) = users {
            return html! {
                <div class=classes!("list")>
                    { u.iter().map(|user| self.view_users(user)).collect::<Html>() }
                </div>
            };
        } else {
            return html! {
                <div class=classes!("loading")>{"loading..."}</div>
            };
        }
    }

    fn view_users(&self, usr: &User) -> Html {
        let admin = if usr.site_admin {
            Some("admin")
        } else {
            Some("")
        };
        return html! {
            <div class=classes!("list-item")>
                <span class=classes!(admin)>
                    <Anchor route=AppRoute::Detail(usr.id as i32)>
                        { &usr.login }
                    </Anchor>
                </span>
            </div>
        };
    }
}

pub enum Msg {}

impl Component for List {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn view(&self) -> Html {
        return html! {
            <div>
                { self.render_list(&self.props.users)}
            </div>
        };
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}
