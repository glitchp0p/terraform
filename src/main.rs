//element combination code for terraform / terraforge game - in conjunction with Claude

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
    
    dict.insert(ElementType::Earth, vec!["stone", "soil", "rock", "dirt", "clay", "ground"]);
    dict.insert(ElementType::Fire, vec!["flame", "heat", "ember", "blaze", "searing", "burn"]);
    dict.insert(ElementType::Water, vec!["liquid", "flow", "stream", "wet", "aqua", "fluid"]);
    dict.insert(ElementType::Air, vec!["wind", "breath", "sky", "gas", "breeze", "vapor"]);
    dict.insert(ElementType::Life, vec!["living", "growth", "vital", "organic", "bio", "fertile"]);
    dict.insert(ElementType::Slime, vec!["goo", "ooze", "gel", "sticky", "viscous", "blob"]);
    dict.insert(ElementType::Compressed, vec!["dense", "pressed", "solid", "compact", "hard", "tight"]);
    dict.insert(ElementType::Heated, vec!["warm", "hot", "burning", "thermal", "molten", "searing"]);
    dict.insert(ElementType::Light, vec!["bright", "brilliant", "aura", "incandecent", "pulsing", "blinding"]);
    dict.insert(ElementType::Toxic, vec!["infected", "unpleasant", "uncomfortable", "poisonous", "shifty", "disquieting"]);
    // ... more elements

    
    dict
}

#[derive(Debug)]
struct Element {
    components: Vec<ElementType>,
    amount: f32, 
}


fn combine_elements(elem1: Element, elem2: Element, elem3: Element) -> Element {
    // Collect all components in order
    let mut all_components = Vec::new();
    all_components.extend(elem1.components);
    all_components.extend(elem2.components);
    all_components.extend(elem3.components);
    
    // Combine amounts
    let total_amount = elem1.amount + elem2.amount + elem3.amount;
    
    Element {
        components: all_components,
        amount: total_amount,
    }
}


fn create_random_element() -> Element {
    let mut rng = rand::thread_rng();
    
    let element_types = [
        ElementType::Earth, ElementType::Water, ElementType::Fire, 
        ElementType::Air, ElementType::Life, ElementType::Slime,
        ElementType::Compressed, ElementType::Heated, ElementType::Frozen, 
        ElementType::Flowing, ElementType::Growing, ElementType::Light, 
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
            let component_names: Vec<String> = self.components
                .iter()
                .map(|c| format!("{:?}", c))
                .collect();
            
            format!("{} [{:.1}]", component_names.join(" + "), self.amount)
        }
    }
}

fn main() {
    let elem1 = create_random_element();
    let elem2 = create_random_element(); 
    let elem3 = create_random_element();
    
    println!("Combining: {}", elem1.display());
    println!("With: {}", elem2.display());
    println!("And: {}", elem3.display());
    
    let result = combine_elements(elem1, elem2, elem3);
    println!("Result: {}", result.display());
}
