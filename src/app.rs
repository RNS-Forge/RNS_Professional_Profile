use yew::prelude::*;
use crate::components::{head::Head, middle_head::MiddleHead, top::Top, news::News, warning::Warning, footer::Footer, sidebar::Sidebar};
use crate::layouts::{screen::Screen};
#[function_component]
pub fn App() -> Html {
    html! {
        <div class="App">
            <Sidebar />
            <Warning />
            <Screen>
                <Top />
                <Head />
                <News />
                <Footer />
            </Screen>
        </div>
    }
}


