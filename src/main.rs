//element combination code for terraform / terraforge game - in conjunction with Claude

use rand::Rng;

#[derive(Debug, PartialEq, Clone)]
enum BaseElement {
    Earth,
    Water,
    Fire,
    Air,
    Life,
    Slime,
    Light,
}

#[derive(Debug, PartialEq, Clone)]
enum Tag {
    Compressed,
    Heated,
    Frozen,
    Flowing,
    Growing,
    Expanding,
    Toxic,
}

#[derive(Debug, PartialEq)]
struct TagIntensity {
    tag: Tag,
    value: u8, // 1, 2, or 3
}

#[derive(Debug)]
struct Element {
    base: BaseElement,
    tags: Vec<TagIntensity>,
    amount: f32, 
}



fn combine_elements(elem1: Element, elem2: Element, elem3: Element) -> Element {
    // For now, just keep the first element's base
    let result_base = elem1.base;
    
    // For now, just keep the first element's tags (we'll improve this)
    let mut merged_tags = elem1.tags;
    
    let result_amount = elem1.amount + elem2.amount + elem3.amount;
    
        // add  tags
   for element in [elem2, elem3] {
        for new_tag_intensity in element.tags {
            //check if tag type exists
            if let Some(exisiting_tag) = merged_tags.iter_mut().find(|t| t.tag == new_tag_intensity.tag) {
                //same tag exisits! add intensities
                exisiting_tag.value += new_tag_intensity.value;
            } else {
                //new tag type, just add it
                merged_tags.push(new_tag_intensity);
            }
        }
    }

    // Simple amount combination
   
    Element {
        base: result_base,
        tags: merged_tags,
        amount: result_amount,
    }
}

fn create_random_element() -> Element {
    let mut rng = rand::thread_rng();
    
    // Pick random base element
    let bases = [BaseElement::Earth, BaseElement::Water, BaseElement::Fire, BaseElement::Air, BaseElement::Life];
    let random_base = bases[rng.gen_range(0..bases.len())].clone();
    
    // Pick random tag and intensity
    let tags = [Tag::Compressed, Tag::Heated, Tag::Frozen, Tag::Flowing, Tag::Growing];
    let random_tag = tags[rng.gen_range(0..tags.len())].clone();
    let random_intensity = rng.gen_range(1..=3);
    let random_amount = rng.gen_range(1.0..10.0);
    
    Element {
        base: random_base,
        tags: vec![TagIntensity { tag: random_tag, value: random_intensity }],
        amount: random_amount,
    }
}

impl Element {
    fn display(&self) -> String {
        let base_name = format!("{:?}", self.base);
        
        let tag_descriptions: Vec<String> = self.tags
            .iter()
            .map(|t| format!("{:?}({})", t.tag, t.value))
            .collect();
        
        if tag_descriptions.is_empty() {
            format!("{} [{:.1}]", base_name, self.amount)
        } else {
            format!("{} + {} [{:.1}]", base_name, tag_descriptions.join(" + "), self.amount)
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
