use yew::prelude::*;
use stylist::style;

// A simple LCG (Linear Congruential Generator) for lightweight deterministic random numbers in WASM
struct SimpleRand {
    state: u32,
}

impl SimpleRand {
    fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        self.state & 0x7fffffff
    }

    fn next_range(&mut self, min: usize, max: usize) -> usize {
        let diff = max - min + 1;
        min + (self.next() as usize % diff)
    }
}

#[derive(Clone, PartialEq)]
struct PlacedWord {
    word: String,
    coords: Vec<(usize, usize)>,
    found: bool,
    color: &'static str,
}

#[derive(Properties, PartialEq)]
pub struct PuzzleProps {
    pub on_close: Callback<()>,
}

#[function_component(PuzzleGame)]
pub fn puzzle_game(props: &PuzzleProps) -> Html {
    let container_style = style!(
        r#"
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background: url('/public/bg/texture.jpg') !important;
        background-repeat: no-repeat;
        background-size: cover;
        z-index: 300;
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 2rem;
        box-sizing: border-box;
        overflow-y: auto;
        font-family: 'Times New Roman', serif;
        "#
    ).unwrap();

    let grid_wrapper = style!(
        r#"
        display: grid;
        grid-template-columns: repeat(10, 40px);
        grid-template-rows: repeat(10, 40px);
        gap: 2px;
        border: 4px double #18181b;
        background: #18181b;
        padding: 4px;
        margin: 1.5rem 0;
        "#
    ).unwrap();

    let cell_style = style!(
        r#"
        width: 40px;
        height: 40px;
        display: flex;
        justify-content: center;
        align-items: center;
        background: #f4f4f5;
        font-family: 'Courier New', monospace;
        font-size: 1.25rem;
        font-weight: bold;
        cursor: pointer;
        user-select: none;
        transition: background 0.2s ease;
        &:hover {
            background: #e4e4e7;
        }
        "#
    ).unwrap();

    let word_list_style = style!(
        r#"
        display: flex;
        flex-wrap: wrap;
        gap: 1rem;
        justify-content: center;
        margin: 1rem 0;
        max-width: 600px;
        "#
    ).unwrap();

    let win_banner_style = style!(
        r#"
        border: 2px dashed #B93C12;
        background: rgba(185, 60, 18, 0.05);
        padding: 1rem 2rem;
        text-align: center;
        margin: 1rem 0;
        "#
    ).unwrap();

    let close_btn_style = style!(
        r#"
        margin-top: 1rem;
        border: 2px solid #18181b;
        padding: 0.5rem 1.5rem;
        background: transparent;
        font-weight: bold;
        cursor: pointer;
        text-transform: uppercase;
        box-shadow: 3px 3px 0px #18181b;
        &:hover {
            background: #B93C12;
            color: #fff;
            box-shadow: 3px 3px 0px #B93C12;
        }
        "#
    ).unwrap();

    // Game State
    let grid = use_state(|| vec![vec![' '; 10]; 10]);
    let placed_words = use_state(|| Vec::<PlacedWord>::new());
    let selected_start = use_state(|| Option::<(usize, usize)>::None);
    let win_state = use_state(|| false);
    let initialized = use_state(|| false);

    // Initialize Game
    let init_game = {
        let grid = grid.clone();
        let placed_words = placed_words.clone();
        let selected_start = selected_start.clone();
        let win_state = win_state.clone();

        Callback::from(move |_| {
            selected_start.set(None);
            win_state.set(false);

            let mut rand = SimpleRand::new(js_sys::Date::now() as u32);
            
            let all_pool = vec![
                "PYTHON", "YEW", "RUST", "AGENT", "DEVELOPER", "BUREAU", "DISPATCH", "TELEGRAM", "ENGINEER"
            ];
            
            // Highlight colors matching reference images
            let colors = vec![
                "rgba(239, 68, 68, 0.3)",   // Red
                "rgba(59, 130, 246, 0.3)",  // Blue
                "rgba(16, 185, 129, 0.3)",  // Green
                "rgba(139, 92, 246, 0.3)",  // Purple
                "rgba(236, 72, 153, 0.3)",  // Pink
                "rgba(245, 158, 11, 0.3)"   // Amber
            ];

            let mut temp_grid = vec![vec!['.'; 10]; 10];
            let mut temp_placed = Vec::<PlacedWord>::new();

            for (idx, word_str) in all_pool.iter().enumerate() {
                let word_len = word_str.len();
                let mut placed = false;
                let color = colors[idx % colors.len()];

                for _ in 0..100 {
                    if placed { break; }
                    let dir_x: i32 = rand.next_range(0, 2) as i32 - 1; // -1, 0, 1
                    let dir_y: i32 = rand.next_range(0, 2) as i32 - 1;
                    if dir_x == 0 && dir_y == 0 { continue; }

                    let start_x = rand.next_range(0, 9);
                    let start_y = rand.next_range(0, 9);

                    // Check boundaries
                    let end_x = start_x as i32 + dir_x * (word_len as i32 - 1);
                    let end_y = start_y as i32 + dir_y * (word_len as i32 - 1);

                    if end_x < 0 || end_x >= 10 || end_y < 0 || end_y >= 10 { continue; }

                    // Check collision
                    let mut conflict = false;
                    let mut coords = Vec::<(usize, usize)>::new();
                    for i in 0..word_len {
                        let cur_x = (start_x as i32 + dir_x * i as i32) as usize;
                        let cur_y = (start_y as i32 + dir_y * i as i32) as usize;
                        let letter = word_str.chars().nth(i).unwrap();

                        if temp_grid[cur_y][cur_x] != '.' && temp_grid[cur_y][cur_x] != letter {
                            conflict = true;
                            break;
                        }
                        coords.push((cur_x, cur_y));
                    }

                    if !conflict {
                        for i in 0..word_len {
                            let (cx, cy) = coords[i];
                            temp_grid[cy][cx] = word_str.chars().nth(i).unwrap();
                        }
                        temp_placed.push(PlacedWord {
                            word: word_str.to_string(),
                            coords,
                            found: false,
                            color,
                        });
                        placed = true;
                    }
                }
            }

            // Fill empty cells
            let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
            for y in 0..10 {
                for x in 0..10 {
                    if temp_grid[y][x] == '.' {
                        let idx = rand.next_range(0, 25);
                        temp_grid[y][x] = letters.chars().nth(idx).unwrap();
                    }
                }
            }

            grid.set(temp_grid);
            placed_words.set(temp_placed);
        })
    };

    // Auto-trigger on first mount
    {
        let init_game = init_game.clone();
        let initialized = initialized.clone();
        use_effect(move || {
            if !*initialized {
                init_game.emit(());
                initialized.set(true);
            }
            || ()
        });
    }

    // Cell Click Handler
    let on_cell_click = {
        let selected_start = selected_start.clone();
        let placed_words = placed_words.clone();
        let win_state = win_state.clone();

        Callback::from(move |(x, y): (usize, usize)| {
            match *selected_start {
                None => {
                    selected_start.set(Some((x, y)));
                }
                Some((sx, sy)) => {
                    // Check if selections match any placed words
                    let mut temp_placed = (*placed_words).clone();
                    let mut found_any = false;

                    for pw in temp_placed.iter_mut() {
                        if pw.found { continue; }
                        // Verify coordinates match forward or reverse
                        let matches_forward = pw.coords.first() == Some(&(sx, sy)) && pw.coords.last() == Some(&(x, y));
                        let matches_reverse = pw.coords.last() == Some(&(sx, sy)) && pw.coords.first() == Some(&(x, y));

                        if matches_forward || matches_reverse {
                            pw.found = true;
                            found_any = true;
                            break;
                        }
                    }

                    if found_any {
                        placed_words.set(temp_placed.clone());
                        // Check win condition
                        let all_found = temp_placed.iter().all(|pw| pw.found);
                        if all_found {
                            win_state.set(true);
                        }
                    }

                    selected_start.set(None);
                }
            }
        })
    };

    // Color cell backgrounds if they are part of a found word or currently selected
    let get_cell_bg = {
        let selected_start = selected_start.clone();
        let placed_words = placed_words.clone();
        
        move |x: usize, y: usize| -> String {
            // If currently selected start cell
            if Some((x, y)) == *selected_start {
                return "background: #fde047;".to_string(); // Bright yellow
            }

            // Check if part of any found word
            for pw in (*placed_words).iter() {
                if pw.found && pw.coords.contains(&(x, y)) {
                    return format!("background: {};", pw.color);
                }
            }

            "background: #f4f4f5;".to_string()
        }
    };

    html! {
        <div class={container_style.get_class_name().to_string()}>
            <div style="text-align: center; margin-bottom: 1rem;">
                <h1 style="font-size: 3rem; font-family: 'OldLondon', serif; border-bottom: 3px solid #18181b; padding-bottom: 0.5rem; text-transform: uppercase;">
                    {"The Sanjay Times — Weekly Word Search"}
                </h1>
                <p style="font-size: 1.10rem; font-style: italic; margin-top: 0.25rem;">
                    {"Find the hidden keywords from the newspaper edition below."}
                </p>
                <p style="font-size: 0.9rem; color: #71717a; margin-top: 0.5rem;">
                    {"Instructions: Click the start letter cell, then click the end letter cell to submit a word."}
                </p>
            </div>

            // Target words
            <div class={word_list_style.get_class_name().to_string()}>
                {
                    (*placed_words).iter().map(|pw| {
                        let text_decoration = if pw.found { "line-through; color: #71717a; opacity: 0.5;" } else { "none;" };
                        html! {
                            <span style={format!("font-size: 1.15rem; font-weight: bold; text-transform: uppercase; font-family: 'Courier New', monospace; text-decoration: {}", text_decoration)}>
                                { &pw.word }
                            </span>
                        }
                    }).collect::<Html>()
                }
            </div>

            // Win state banner
            if *win_state {
                <div class={win_banner_style.get_class_name().to_string()}>
                    <h2 style="font-size: 1.5rem; font-weight: bold; color: #B93C12;">
                        {"CONGRATULATIONS — EDITION SOLVED!"}
                    </h2>
                    <p style="font-size: 0.95rem; margin-top: 0.25rem;">
                        {"You have successfully cataloged all keywords from today's publication dispatch."}
                    </p>
                </div>
            }

            // Word Search Grid
            <div class={grid_wrapper.get_class_name().to_string()}>
                {
                    (*grid).iter().enumerate().map(|(y, row)| {
                        row.iter().enumerate().map(|(x, &letter)| {
                            let click_handler = on_cell_click.clone();
                            let bg = get_cell_bg(x, y);
                            html! {
                                <div 
                                    onclick={Callback::from(move |_| click_handler.emit((x, y)))}
                                    class={cell_style.get_class_name().to_string()} 
                                    style={bg}
                                >
                                    { letter }
                                </div>
                            }
                        }).collect::<Html>()
                    }).collect::<Html>()
                }
            </div>

            <div style="display: flex; gap: 1rem;">
                <button onclick={let init = init_game.clone(); Callback::from(move |_: MouseEvent| init.emit(()))} class={close_btn_style.get_class_name().to_string()}>
                    {"Regenerate Grid"}
                </button>
                <button onclick={let on_close = props.on_close.clone(); Callback::from(move |_: MouseEvent| on_close.emit(()))} class={close_btn_style.get_class_name().to_string()}>
                    {"Return to Gazette"}
                </button>
            </div>
        </div>
    }
}
