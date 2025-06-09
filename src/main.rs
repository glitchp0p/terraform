#[derive(Debug)]
enum BaseElement {
    Earth,
    Water,
    Fire,
    Air,
    Life,
}

#[derive(Debug)]
enum Tag {
    Compressed,
    Heated,
    Frozen,
    Flowing,
    Growing,
}

#[derive(Debug)]
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

fn main() {
    // Test creating an element
    let rock = Element {
        base: BaseElement::Earth,
        tags: vec![TagIntensity { tag: Tag::Compressed, level: 2 }],
        amount: 10.0,
    };
    
    println!("Created element: {:?}", rock);
}
