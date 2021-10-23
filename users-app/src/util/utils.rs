use yew_router::components::RouterAnchor;
use yew_router::Switch;

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/{id}"]
    Detail(i32),
    #[to = "/"]
    Home,
}

pub type Anchor = RouterAnchor<AppRoute>;
