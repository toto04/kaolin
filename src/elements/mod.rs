pub mod flexbox;
pub mod text;

use flexbox::FlexBox;
use text::TextElement;

use crate::{
    commands::RenderCommand,
    style::sizing::{PreferredSize, SizingDimensions},
};
use uuid::Uuid;

pub enum KaolinElement {
    Flex(FlexBox),
    Text(TextElement),
}

struct KaolinNode {
    id: String,
    growable_width: bool,
    growable_height: bool,
    shrinkable: bool,
    element: KaolinElement,
    sizing: (SizingDimensions, SizingDimensions),
    current_width: f32,
    current_height: f32,
    x: f32,
    y: f32,
}

impl KaolinNode {
    fn new(element: KaolinElement, id: Option<String>) -> Self {
        let id = id.unwrap_or_else(|| Uuid::new_v4().to_string());
        match &element {
            KaolinElement::Flex(flex_box) => {
                let style = &flex_box.style;
                let width = SizingDimensions::from(style.sizing.width);
                let height = SizingDimensions::from(style.sizing.height);

                KaolinNode {
                    id,
                    growable_width: width.is_growable(),
                    growable_height: height.is_growable(),
                    shrinkable: width.is_shrinkable(),
                    current_width: width.clamped(flex_box.fit_width_to_children()),
                    current_height: 0.0,
                    sizing: (width, height),
                    element,
                    x: 0.0,
                    y: 0.0,
                }
            }
            KaolinElement::Text(text_element) => {
                let (pref_width, pref_height) = text_element.get_preferred_size();
                let width = SizingDimensions {
                    min: text_element.get_minimum_size().0,
                    preferred: PreferredSize::Fixed(pref_width),
                    max: pref_width,
                };
                let height = SizingDimensions {
                    min: pref_height,
                    preferred: PreferredSize::Fixed(pref_height),
                    max: f32::INFINITY,
                };

                KaolinNode {
                    id,
                    growable_width: false,
                    growable_height: false,
                    shrinkable: true,
                    element,
                    current_width: pref_width,
                    current_height: pref_height,
                    sizing: (width, height),
                    x: 0.0,
                    y: 0.0,
                }
            }
        }
    }

    pub fn get_grow_factor(&self) -> (f32, f32) {
        let w_factor = match self.sizing.0.preferred {
            PreferredSize::Grow(factor) => factor,
            _ => 0.0, // Default grow factor if not specified
        };
        let h_factor = match self.sizing.1.preferred {
            PreferredSize::Grow(factor) => factor,
            _ => 0.0, // Default grow factor if not specified
        };
        (w_factor, h_factor)
    }

    fn grow_width(&mut self, limit: f32) -> f32 {
        match self.element {
            KaolinElement::Flex(..) => {
                let grown;
                if limit > self.sizing.0.max - self.current_width {
                    grown = self.sizing.0.max - self.current_width;
                    self.growable_width = false; // No more growth possible
                } else if limit < self.sizing.0.min - self.current_width {
                    grown = self.sizing.0.min - self.current_width;
                    self.shrinkable = false; // No more shrinking possible
                } else {
                    grown = limit;
                }

                self.current_width += grown;
                self.current_width = self.current_width.min(self.sizing.0.max);
                grown
            }
            KaolinElement::Text(..) => {
                let shrunk;
                if limit > 0.0 {
                    shrunk = 0.0; // Text elements do not grow
                } else if limit < self.sizing.0.min - self.current_width {
                    shrunk = self.sizing.0.min - self.current_width;
                    self.shrinkable = false; // No more shrinking possible
                } else {
                    shrunk = limit;
                }
                self.current_width += shrunk;
                shrunk
            }
        }
    }

    fn grow_height(&mut self, limit: f32) -> f32 {
        match self.element {
            KaolinElement::Flex(..) => {
                let grown;
                if limit > self.sizing.1.max - self.current_height {
                    grown = self.sizing.1.max - self.current_height;
                    self.growable_height = false; // No more growth possible
                } else if limit < self.sizing.1.min - self.current_height {
                    grown = self.sizing.1.min - self.current_height;
                    self.shrinkable = false; // No more shrinking possible
                } else {
                    grown = limit;
                }

                self.current_height += grown;
                self.current_height = self.current_height.min(self.sizing.1.max);
                grown
            }
            KaolinElement::Text(..) => {
                let shrunk;
                if limit > 0.0 {
                    shrunk = 0.0; // Text elements do not grow
                } else if limit < self.sizing.1.min - self.current_height {
                    shrunk = self.sizing.1.min - self.current_height;
                    self.shrinkable = false; // No more shrinking possible
                } else {
                    shrunk = limit;
                }
                self.current_height += shrunk;
                shrunk
            }
        }
    }

    fn fit_height(&mut self) {
        if let KaolinElement::Flex(ref mut flex_box) = self.element {
            self.current_height =
                flex_box.fit_height_to_children(self.current_height, self.sizing.1.max);
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        if let KaolinElement::Flex(ref mut flex_box) = self.element {
            flex_box.position_children((x, x + self.current_width, y, y + self.current_height));
        }
    }

    pub fn render(&self) -> Vec<RenderCommand> {
        match &self.element {
            KaolinElement::Flex(flex_box) => {
                flex_box.render_all(vec![RenderCommand::DrawRectangle {
                    id: self.id.as_str(),
                    x: self.x as i32,
                    y: self.y as i32,
                    width: self.current_width as i32,
                    height: self.current_height as i32,
                    color: flex_box.style.background_color,
                }])
            }
            KaolinElement::Text(text_element) => text_element.render(self.x, self.y),
        }
    }
}

struct KaolinNodes {
    nodes: Vec<KaolinNode>,
}

impl KaolinNodes {
    fn new() -> Self {
        KaolinNodes { nodes: Vec::new() }
    }

    fn push(&mut self, node: KaolinNode) {
        self.nodes.push(node);
    }

    fn get_growable_children_w(&mut self) -> Vec<&mut KaolinNode> {
        self.nodes.iter_mut().filter(|c| c.growable_width).collect()
    }

    fn get_growable_children_h(&mut self) -> Vec<&mut KaolinNode> {
        self.nodes
            .iter_mut()
            .filter(|c| c.growable_height)
            .collect()
    }

    fn get_shrinkable_children(&mut self) -> Vec<&mut KaolinNode> {
        self.nodes.iter_mut().filter(|c| c.shrinkable).collect()
    }

    fn get_smallest_widths(children: &Vec<&mut KaolinNode>) -> (f32, f32) {
        let mut smallest = f32::INFINITY;
        let mut second_smallest = f32::INFINITY;
        for node in children {
            if node.current_width < smallest {
                second_smallest = smallest;
                smallest = node.current_width;
            } else if node.current_width < second_smallest && node.current_width > smallest {
                second_smallest = node.current_width;
            }
        }
        (smallest, second_smallest)
    }

    fn get_smallest_heights(children: &Vec<&mut KaolinNode>) -> (f32, f32) {
        let mut smallest = f32::INFINITY;
        let mut second_smallest = f32::INFINITY;
        for node in children {
            if node.current_height < smallest {
                second_smallest = smallest;
                smallest = node.current_height;
            } else if node.current_height < second_smallest && node.current_height > smallest {
                second_smallest = node.current_height;
            }
        }
        (smallest, second_smallest)
    }

    fn get_biggest_widths(children: &Vec<&mut KaolinNode>) -> (f32, f32) {
        let mut biggest = 0.0;
        let mut second_biggest = 0.0;
        for node in children {
            if node.current_width > biggest {
                second_biggest = biggest;
                biggest = node.current_width;
            } else if node.current_width > second_biggest && node.current_width < biggest {
                second_biggest = node.current_width;
            }
        }
        (biggest, second_biggest)
    }

    fn get_cumulative_width(&self) -> f32 {
        self.nodes.iter().map(|c| c.current_width).sum()
    }

    fn get_max_width(&self) -> f32 {
        self.nodes
            .iter()
            .fold(0.0, |acc, c| acc.max(c.current_width))
    }

    fn get_cumulative_height(&self) -> f32 {
        self.nodes.iter().map(|c| c.current_height).sum()
    }

    fn get_max_height(&self) -> f32 {
        self.nodes
            .iter()
            .fold(0.0, |acc, c| acc.max(c.current_height))
    }

    fn nodes(&mut self) -> impl Iterator<Item = &mut KaolinNode> {
        self.nodes.iter_mut()
    }

    pub fn grow_w(&mut self) {
        self.nodes.iter_mut().for_each(|c| match c.element {
            KaolinElement::Flex(ref mut flex_box) => {
                flex_box.grow_children_width(c.current_width);
                c.fit_height(); // height fit depends on children (text elements will wrap upon width growth)
            }
            KaolinElement::Text(ref mut text_element) => {
                c.current_height = text_element.wrap_text(c.current_width); // height gets fit to the text itself
            }
        });
    }

    pub fn grow_h(&mut self) {
        self.nodes.iter_mut().for_each(|c| {
            if let KaolinElement::Flex(ref mut flex_box) = c.element {
                flex_box.grow_children_height(c.current_height);
            }
        });
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}
