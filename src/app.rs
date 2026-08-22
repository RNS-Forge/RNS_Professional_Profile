use yew::prelude::*;
use crate::components::{head::Head, middle_head::MiddleHead, top::Top, news::News, warning::Warning, footer::Footer, sidebar::Sidebar, puzzle::PuzzleGame, projects_archive::ProjectsArchive, newspaper_intro::NewspaperIntro};
use crate::layouts::{screen::Screen};

#[function_component]
pub fn App() -> Html {
    let show_puzzle = use_state(|| false);
    let show_archive = use_state(|| false);
    let show_intro = use_state(|| true);

    let toggle_puzzle = {
        let show_puzzle = show_puzzle.clone();
        Callback::from(move |_: ()| {
            gloo::console::log!(format!("App: toggle_puzzle called. Toggling show_puzzle from {} to {}", *show_puzzle, !*show_puzzle));
            show_puzzle.set(!*show_puzzle);
        })
    };

    let toggle_archive = {
        let show_archive = show_archive.clone();
        Callback::from(move |_: ()| {
            show_archive.set(!*show_archive);
        })
    };

    let on_intro_complete = {
        let show_intro = show_intro.clone();
        Callback::from(move |_: ()| {
            show_intro.set(false);
        })
    };

    html! {
        <div class="App">
            if *show_intro {
                <NewspaperIntro on_complete={on_intro_complete} />
            }
            <Sidebar on_play_puzzle={toggle_puzzle.clone()} />
            <Warning />
            if *show_puzzle {
                <PuzzleGame on_close={toggle_puzzle.clone()} />
            }
            if *show_archive {
                <ProjectsArchive on_close={toggle_archive.clone()} />
            }
            <Screen>
                <Top />
                <Head />
                <News on_view_archive={toggle_archive.clone()} />
                <Footer />
            </Screen>
        </div>
    }
}


