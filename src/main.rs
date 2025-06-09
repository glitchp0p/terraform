#[derive(Debug)]
enum BaseElement {
    Earth,
    Water,
    Fire,
    Air,
    Life,
}

#[derive(Debug, PartialEq)]
enum Tag {
    Compressed,
    Heated,
    Frozen,
    Flowing,
    Growing,
}

#[derive(Debug, PartialEq)]
struct TagIntensity {
    tag: Tag,
    level: u8, // 1, 2, or 3
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
                exisiting_tag.level += new_tag_intensity.level;
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

fn main() {
    let earth = Element { 
        base: BaseElement::Earth, 
        tags: vec![TagIntensity { tag: Tag::Compressed, level: 2 }], 
        amount: 5.0 
    };
    
    let fire = Element { 
        base: BaseElement::Fire, 
        tags: vec![TagIntensity { tag: Tag::Heated, level: 3 }], 
        amount: 3.0 
    };
    
    let air = Element {
        base: BaseElement::Air,
        tags: vec![TagIntensity { tag: Tag::Flowing, level: 2}],
        amount: 9.0
    };

    let result = combine_elements(earth, fire, air);
    println!("Combined result: {:?}", result);
}
