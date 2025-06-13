//element combination code for terraform / terraforge game - in conjunction with Claude

use std::collections::HashSet;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use std::io::{self, stdout};
use std::time::{Duration, Instant};

mod time;
use time::TimeSystem;

use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum ElementType {
    //base elements
    Earth,
    Water,
    Fire,
    Air,
    Life,
    Slime,
    Light,
    Crystal,
    //properties
    Compressed,
    Heated,
    Frozen,
    Flowing,
    Growing,
    Toxic,
}

fn create_synonym_dict() -> HashMap<ElementType, Vec<&'static str>> {
    let mut dict = HashMap::new();

    dict.insert(
        ElementType::Earth,
        vec!["stone", "soil", "rock", "dirt", "clay", "ground"],
    );
    dict.insert(
        ElementType::Fire,
        vec!["flame", "heat", "ember", "blaze", "searing", "burn"],
    );
    dict.insert(
        ElementType::Water,
        vec!["liquid", "flow", "stream", "wet", "aqua", "fluid"],
    );
    dict.insert(
        ElementType::Air,
        vec!["wind", "breath", "sky", "gas", "breeze", "vapor"],
    );
    dict.insert(
        ElementType::Life,
        vec!["living", "growth", "vital", "organic", "bio", "fertile"],
    );
    dict.insert(
        ElementType::Slime,
        vec![
            "gooey", "oozing", "jelly", "sticky", "viscous", "blob", "pudding",
        ],
    );
    dict.insert(
        ElementType::Light,
        vec![
            "bright",
            "brilliant",
            "aura",
            "incandecent",
            "pulsing",
            "blinding",
        ],
    );
    dict.insert(
        ElementType::Compressed,
        vec!["dense", "pressed", "solid", "compact", "hard", "tight"],
    );

    dict.insert(
        ElementType::Heated,
        vec!["warm", "hot", "burning", "thermal", "molten", "searing"],
    );
    dict.insert(
        ElementType::Toxic,
        vec![
            "infected",
            "unpleasant",
            "uncomfortable",
            "poisonous",
            "shifty",
            "disquieting",
        ],
    );
    dict.insert(
        ElementType::Frozen,
        vec!["icy", "frigid", "cold", "refrigerated", "bracing", "arctic"],
    );
    dict.insert(
        ElementType::Crystal,
        vec![
            "crystaline",
            "faceted",
            "gem",
            "mirror",
            "clean",
            "polygonal",
            "angular",
        ],
    );
    dict.insert(ElementType::Flowing, vec!["fluid", "drifting", "qi", "energetic", "ambient", "spreading"]);
    dict.insert(ElementType::Growing, vec!["large", "expansive", "monstrous", "majestic", "open"]);

    // ... more elements

    dict
}

#[derive(Debug)]
struct Element {
    components: Vec<ElementType>,
    amount: f32,
}

fn combine_elements(elem1: Element, elem2: Element, elem3: Element) -> Element {
    use std::collections::HashMap;

    // Collect all components in order
    let mut all_components = Vec::new();
    all_components.extend(elem1.components);
    all_components.extend(elem2.components);
    all_components.extend(elem3.components);

    // merge duplicates and sum amounts
    let mut element_totals = HashMap::new(); 
    // Combine amounts
    let total_amount = elem1.amount + elem2.amount + elem3.amount;

    // for now assume each component gets equal share of amount
    let amount_per_component = total_amount / all_components.len() as f32;

    for component in all_components {
        *element_totals.entry(component).or_insert(0.0) += amount_per_component;
    }

    // convert back to components for now, just keep the element_types
    let merged_components: Vec<ElementType> = element_totals.keys().cloned().collect();

    Element {
        components: merged_components,
        amount: total_amount,
    }
}

fn create_random_element() -> Element {
    let mut rng = rand::thread_rng();

    let element_types = [
        ElementType::Earth,
        ElementType::Water,
        ElementType::Fire,
        ElementType::Air,
        ElementType::Life,
        ElementType::Slime,
        ElementType::Compressed,
        ElementType::Heated,
        ElementType::Frozen,
        ElementType::Flowing,
        ElementType::Growing,
        ElementType::Light,
        ElementType::Toxic,
    ];

    let random_element = element_types[rng.gen_range(0..element_types.len())].clone();
    let random_amount = rng.gen_range(1.0..10.0);

    Element {
        components: vec![random_element],
        amount: random_amount,
    }
}

impl Element {
    fn display(&self) -> String {
        if self.components.is_empty() {
            format!("Empty [{:.1}]", self.amount)
        } else {
            let component_names: Vec<String> =
                self.components.iter().map(|c| format!("{:?}", c)).collect();

            format!("{} [{:.1}]", component_names.join(" + "), self.amount)
        }
    }
    fn display_with_names(&self) -> String {
        if self.components.is_empty() {
            format!("Empty [{:.1}]", self.amount)
        } else {
            let dict = create_synonym_dict();
            let name = generate_element_name(&self.components, &dict);
            format!("{} [{:.1}]", name, self.amount)
        }
    }
}

fn generate_element_name(
    components: &[ElementType],
    dict: &HashMap<ElementType, Vec<&str>>,
) -> String {
    // Start simple - just take first 2 components
    if components.is_empty() {
        return "Void".to_string();
    }

    if components.len() == 1 {
        // Single component - pick first synonym
        if let Some(synonyms) = dict.get(&components[0]) {
            return synonyms[0].to_string();
        }
        return format!("{:?}", components[0]);
    }

    // Multiple components - combine first two synonyms
    let first_name = dict
        .get(&components[0])
        .map(|syns| syns[0])
        .unwrap_or("Unknown");

    let second_name = dict
        .get(&components[1])
        .map(|syns| syns[0])
        .unwrap_or("Unknown");

    format!("{} {}ness", first_name, second_name)
}


fn main() -> io::Result<()> {
    //enable raw mode for real time key input
    terminal::enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    
    // let elem1 = create_random_element();
    // let elem2 = create_random_element();
    // let elem3 = create_random_element();
    //
    // println!(
    //     "Combining: {} ({})",
    //     elem1.display_with_names(),
    //     elem1.display()
    // );
    // println!("With: {} ({})", elem2.display_with_names(), elem2.display());
    // println!("And: {} ({})", elem3.display_with_names(), elem3.display());
    //
    // let result = combine_elements(elem1, elem2, elem3);
    // println!(
    //     "Result: {} ({})",
    //     result.display_with_names(),
    //     result.display()
    // );
    

    // Time system first implementation
    
    let mut time_system = TimeSystem::new();
    let mut last_time = Instant::now();
    let mut held_keys = HashSet::new();

    println!("Time System Test - press 'q' to quit, left/right arrows control time\r");

    loop {
        let now = Instant::now();
        let delta_time = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        //check for keyboard input
        let (left_held, right_held, should_quit) = check_input(&mut held_keys)?;

        if should_quit {
            break;
        }
    

        time_system.update(delta_time, right_held, left_held);

       // println!("Tick: {}, Rate: {:.2}, Hold Duration: {:.2}",
       //     time_system.current_tick,
       //     time_system.tick_rate,
       //     time_system.key_hold_duration);

        std::thread::sleep(Duration::from_millis(200)); //50ms = 20 fps update

        //added to debug input errors
        println!("Tick: {}, Rate: {:.2}, Left: {}, Right: {}\r", 
            time_system.current_tick, 
            time_system.tick_rate,
            left_held,
            right_held);

    }

    //cleanup 
    terminal::disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}


fn check_input(held_keys: &mut HashSet<KeyCode>) -> io::Result<(bool, bool, bool)> {
    let mut should_quit = false;
    let mut left_pressed = false;
    let mut right_pressed = false;

    //check for new key presses only
    while event::poll(Duration::from_millis(0))? {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Left => left_pressed = true,
                    KeyCode::Right => right_pressed = true,
                    KeyCode::Char('q') => should_quit = true,
                    _ => {}
                }
            }
        }
    }
    
    Ok((left_pressed, right_pressed, should_quit))
}
