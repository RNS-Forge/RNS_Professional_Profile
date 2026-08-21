use yew::prelude::*;

//pub mod news_items;
// Import your components (you'll need to define these elsewhere)
use crate::components::middle_head::MiddleHead;
use crate::components::news_items::news1::News1;
use crate::components::news_items::news2::News2;
use crate::components::news_items::news4::News4;
use crate::components::education::Education;
use crate::components::projects_page::ProjectsPage;
use crate::components::blog_page::BlogPage;
use crate::components::contact_page::ContactPage;

use crate::components::the_end::TheEnd;

#[function_component(News)]
pub fn news() -> Html {
    html! {
        <>
            <News1 />
            <hr class={classes!("border-t-2", "border-zinc-800")} />
            <MiddleHead />
            <hr class={classes!("border-t-2", "mb-6", "border-zinc-800")} />
            <News2 />
            <hr class={classes!("border-t-2", "my-6", "border-zinc-800")} />
            <News4 />
            <hr class={classes!("border-t-2", "my-6", "border-zinc-800")} />
            <ProjectsPage />
            <hr class={classes!("border-t-2", "my-6", "border-zinc-800")} />
            <Education />
            <hr class={classes!("border-t-2", "my-6", "border-zinc-800")} />
            <BlogPage />
            <hr class={classes!("border-t-2", "my-6", "border-zinc-800")} />
            <ContactPage />
            <hr class={classes!("border-t-2", "my-4", "border-zinc-800")} />
            <TheEnd />
        </>
    }
}

