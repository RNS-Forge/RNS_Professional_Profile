use yew::prelude::*;
use stylist::style;

#[derive(Clone, Properties, PartialEq)]
struct BlogArticleProps {
    title: &'static str,
    date: &'static str,
    category: &'static str,
    snippet: &'static str,
    full_content: &'static str,
}

#[function_component(BlogArticle)]
fn blog_article(props: &BlogArticleProps) -> Html {
    let is_expanded = use_state(|| false);
    
    let toggle_expand = {
        let is_expanded = is_expanded.clone();
        Callback::from(move |_| is_expanded.set(!*is_expanded))
    };

    let item_style = style!(
        r#"
        border-bottom: 2px solid #18181b;
        padding-bottom: 1.5rem;
        margin-bottom: 1.5rem;
        &:last-child {
            border-bottom: none;
        }
        "#
    ).unwrap();

    html! {
        <div class={item_style.get_class_name().to_string()}>
            <span style="font-size: 0.75rem; text-transform: uppercase; font-weight: bold; color: #B93C12; letter-spacing: 1px;">
                {props.category} { " • " } {props.date}
            </span>
            <h3 style="font-size: 1.6rem; font-weight: bold; font-family: 'Times New Roman', serif; margin-top: 0.25rem; margin-bottom: 0.5rem; line-height: 1.2;">
                {props.title}
            </h3>
            
            <p style="font-size: 0.95rem; line-height: 1.5; color: #18181b;">
                { if *is_expanded { props.full_content } else { props.snippet } }
            </p>
            
            <button 
                onclick={toggle_expand}
                style="margin-top: 0.75rem; font-family: 'Times New Roman', serif; font-size: 0.9rem; font-weight: bold; color: #B93C12; border: 1px solid #B93C12; padding: 0.25rem 0.75rem; background: transparent; cursor: pointer; border-radius: 2px;"
            >
                { if *is_expanded { "Read Less [-]" } else { "Read Full Article [+]" } }
            </button>
        </div>
    }
}

#[function_component(BlogPage)]
pub fn blog_page() -> Html {
    let container_style = style!(
        r#"
        margin: 2rem 0;
        text-align: justify;
        "#
    ).unwrap();

    let grid_style = style!(
        r#"
        display: grid;
        grid-template-columns: 2fr 1fr;
        gap: 2rem;
        width: 100%;
        @media (max-width: 768px) {
            grid-template-columns: 1fr;
        }
        "#
    ).unwrap();

    let sidebar_style = style!(
        r#"
        border-left: 2px solid #18181b;
        padding-left: 1.5rem;
        @media (max-width: 768px) {
            border-left: none;
            padding-left: 0;
            border-top: 2px solid #18181b;
            padding-top: 1.5rem;
        }
        "#
    ).unwrap();

    html! {
        <div id="blog-section" class={container_style.get_class_name().to_string()}>
            <h1 style="font-size: 2rem; border-bottom: 2px solid #18181b; padding-bottom: 0.5rem; margin-bottom: 1rem; text-align: center; font-family: 'Times New Roman', serif; text-transform: uppercase;">
                {"The Gazette Columns — Thoughts on Tech & AI"}
            </h1>
            
            <div class={grid_style.get_class_name().to_string()}>
                // Main Blog Feed
                <div>
                    <BlogArticle 
                        title="The Rise of Agentic AI: Moving Beyond Prompt Engineering"
                        date="August 2026"
                        category="Artificial Intelligence"
                        snippet="For the past few years, the AI narrative was dominated by prompts and responses. Today, we stand on the precipice of autonomous execution loops, where intelligent agents coordinate, delegate, and inspect themselves to deliver structured software results..."
                        full_content="For the past few years, the AI narrative was dominated by prompts and responses. Today, we stand on the precipice of autonomous execution loops, where intelligent agents coordinate, delegate, and inspect themselves to deliver structured software results. Rather than typing a single prompt to receive a block of text, developers compile multi-agent systems using tools like LangGraph and CrewAI. These agents maintain state across multiple execution phases, dynamically search web indexes, and write code to address issues. In this new paradigm, developer engineering transitions from writing line-by-line procedural code to orchestrating behavioral bounds for networks of autonomous machine intelligence."
                    />
                    <BlogArticle 
                        title="Why Rust and WebAssembly are the Future of Interactive Portfolios"
                        date="July 2026"
                        category="Web Development"
                        snippet="Web developers are traditionally constrained by Javascript's dynamic execution limits. Compiling Rust code to WebAssembly binaries opens up desktop-tier computation performance in sandbox client browsers, rendering high-fidelity designs..."
                        full_content="Web developers are traditionally constrained by Javascript's dynamic execution limits. Compiling Rust code to WebAssembly binaries opens up desktop-tier computation performance in sandbox client browsers, rendering high-fidelity designs at 60 FPS. Yew provides a robust component-based paradigm resembling React, but backed by Rust's strict compiler guarantees. Combined with Trunk as a bundler, the compilation yields minimal loading bundles, fast execution times, and complete safety against typical client runtime exceptions. As layouts become richer and demand more offline capability, WebAssembly frameworks are positioned to capture complex client interfaces."
                    />
                </div>

                // Sidebar items
                <div class={sidebar_style.get_class_name().to_string()}>
                    <h2 style="font-size: 1.25rem; font-weight: bold; border-bottom: 1px solid #18181b; padding-bottom: 0.25rem; margin-bottom: 0.75rem; font-family: 'Times New Roman', serif; text-transform: uppercase;">
                        {"Editorial Tidbits"}
                    </h2>
                    <p style="font-size: 0.85rem; line-height: 1.4; margin-bottom: 1rem; font-style: italic;">
                        {"Thoughts and minor reflections compiled by the bureau."}
                    </p>
                    <div style="font-size: 0.85rem; line-height: 1.5; display: flex; flex-direction: column; gap: 0.75rem;">
                        <div>
                            <strong style="color: #B93C12;">{"On API Latency:"}</strong>
                            <span>{" Structuring API calls in modular states inside Yew avoids rendering bottlenecks."}</span>
                        </div>
                        <div>
                            <strong style="color: #B93C12;">{"On Model Training:"}</strong>
                            <span>{" Keeping parameters lightweight allows small language models to achieve quick RAG updates."}</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
