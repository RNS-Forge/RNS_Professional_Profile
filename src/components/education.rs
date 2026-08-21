use yew::prelude::*;
use stylist::style;

#[function_component(Education)]
pub fn education() -> Html {
    let container_style = style!(
        r#"
        margin: 2rem 0;
        text-align: justify;
        "#
    ).unwrap();

    let grid_style = style!(
        r#"
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 2rem;
        width: 100%;
        @media (max-width: 768px) {
            grid-template-columns: 1fr;
        }
        "#
    ).unwrap();

    let border_box = style!(
        r#"
        border: 4px double #18181b;
        padding: 1.5rem;
        background: rgba(24, 24, 27, 0.01);
        "#
    ).unwrap();

    html! {
        <div id="education-section" class={container_style.get_class_name().to_string()}>
            <h1 style="font-size: 2rem; border-bottom: 2px solid #18181b; padding-bottom: 0.5rem; margin-bottom: 1rem; text-align: center; font-family: 'Times New Roman', serif; text-transform: uppercase;">
                {"Scholastic Ledger — Education & Honors"}
            </h1>
            
            <div class={grid_style.get_class_name().to_string()}>
                // Education details
                <div class={border_box.get_class_name().to_string()}>
                    <h2 style="font-size: 1.5rem; font-weight: bold; margin-bottom: 1rem; font-family: 'Times New Roman', serif;">
                        {"I. Academic Enrollment"}
                    </h2>
                    
                    <div style="margin-bottom: 1.5rem;">
                        <h3 style="font-size: 1.2rem; font-weight: bold; color: #B93C12;">
                            {"B.Tech in Artificial Intelligence & Machine Learning"}
                        </h3>
                        <p style="font-style: italic; font-weight: bold; color: #18181b;">
                            {"SNS College of Technology, Coimbatore"}
                        </p>
                        <p style="font-size: 0.9rem; color: #3f3f46; margin-top: 0.25rem;">
                            {"Graduation Year: 2026 | Cumulative Grade Point Average: 8.38 / 10.00"}
                        </p>
                        <p style="font-size: 0.85rem; margin-top: 0.5rem; line-height: 1.4;">
                            {"Focused studies on Agentic AI Frameworks, Neural Network Architectures, Deep Learning optimization, and scalable Full-Stack Software Engineering paradigms."}
                        </p>
                    </div>

                    <div>
                        <h3 style="font-size: 1.1rem; font-weight: bold;">
                            {"Core Academic Modules"}
                        </h3>
                        <ul style="list-style: square inside; font-size: 0.85rem; margin-top: 0.5rem; line-height: 1.5;">
                            <li>{"Design & Analysis of Algorithms"}</li>
                            <li>{"Machine Learning Paradigms & Model Training"}</li>
                            <li>{"Natural Language Processing & RAG Workflows"}</li>
                            <li>{"Database Management Systems & Distributed Storage"}</li>
                        </ul>
                    </div>
                </div>

                // Certifications and Badges
                <div class={border_box.get_class_name().to_string()}>
                    <h2 style="font-size: 1.5rem; font-weight: bold; margin-bottom: 1rem; font-family: 'Times New Roman', serif;">
                        {"II. Professional Accreditations"}
                    </h2>
                    
                    <ul style="list-style: none; padding: 0; display: flex; flex-direction: column; gap: 0.75rem;">
                        <li style="border-bottom: 1px dashed rgba(24,24,27,0.25); padding-bottom: 0.5rem;">
                            <strong style="color: #B93C12; font-size: 1rem;">{"Oracle Cloud Infrastructure (OCI) AI Foundations"}</strong>
                            <p style="font-size: 0.85rem; color: #3f3f46; margin-top: 0.1rem;">
                                {"Score: 92% | Issued by Oracle Corporation"}
                            </p>
                        </li>
                        <li style="border-bottom: 1px dashed rgba(24,24,27,0.25); padding-bottom: 0.5rem;">
                            <strong style="color: #B93C12; font-size: 1rem;">{"Salesforce AI Associate & Agentforce Specialist"}</strong>
                            <p style="font-size: 0.85rem; color: #3f3f46; margin-top: 0.1rem;">
                                {"Validated proficiency (80%+) in autonomous enterprise agents and setup."}
                            </p>
                        </li>
                        <li style="border-bottom: 1px dashed rgba(24,24,27,0.25); padding-bottom: 0.5rem;">
                            <strong style="color: #B93C12; font-size: 1rem;">{"Postman AI Student Expert"}</strong>
                            <p style="font-size: 0.85rem; color: #3f3f46; margin-top: 0.1rem;">
                                {"Certified in API testing and automated requests integration with AI extensions."}
                            </p>
                        </li>
                        <li>
                            <strong style="color: #B93C12; font-size: 1rem;">{"NASSCOM Digital Edge Certification"}</strong>
                            <p style="font-size: 0.85rem; color: #3f3f46; margin-top: 0.1rem;">
                                {"Score: 81% | Certified in FutureSkills Prime analytics program."}
                            </p>
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
