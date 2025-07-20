pub mod flexbox;
pub mod text;

use flexbox::FlexBox;
use text::TextElement;

use crate::style::sizing::{PreferredSize, SizingDimensions};
use uuid::Uuid;

pub enum KaolinElement<'a> {
    Flex(FlexBox<'a>),
    Text(TextElement<'a>),
}

struct KaolinNode<'a> {
    id: String,
    growable_width: bool,
    growable_height: bool,
    shrinkable: bool,
    element: KaolinElement<'a>,
    sizing: (SizingDimensions, SizingDimensions),
    current_width: f32,
    current_height: f32,
}

impl<'a> KaolinNode<'a> {
    fn new(element: KaolinElement<'a>, id: Option<String>) -> Self {
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
                    current_width: width.clamped(flex_box.fit_to_width()),
                    current_height: 0.0,
                    sizing: (width, height),
                    element,
                }
            }
            KaolinElement::Text(_) => {
                let width = SizingDimensions::default();
                let height = SizingDimensions::default();

                KaolinNode {
                    id,
                    growable_width: false,
                    growable_height: false,
                    shrinkable: true,
                    element,
                    current_width: 0.0,
                    current_height: 0.0,
                    sizing: (width, height),
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
                self.current_width = (self.current_width + limit).min(self.sizing.0.max);
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

    fn fit_height(&mut self) {
        if let KaolinElement::Flex(ref mut flex_box) = self.element {
            self.current_height =
                flex_box.fit_height_to_children(self.current_height, self.sizing.1.max);
        }
    }
}

struct KaolinNodes<'a> {
    nodes: Vec<KaolinNode<'a>>,
}

impl<'a> KaolinNodes<'a> {
    fn new() -> Self {
        KaolinNodes { nodes: Vec::new() }
    }

    fn push(&mut self, node: KaolinNode<'a>) {
        self.nodes.push(node);
    }

    fn get_growable_children_w(&mut self) -> Vec<&mut KaolinNode<'a>> {
        self.nodes.iter_mut().filter(|c| c.growable_width).collect()
    }

    fn get_shrinkable_children(&mut self) -> Vec<&mut KaolinNode<'a>> {
        self.nodes.iter_mut().filter(|c| c.shrinkable).collect()
    }

    fn get_smallest_widths(children: &Vec<&mut KaolinNode<'a>>) -> (f32, f32) {
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

    fn nodes(&self) -> impl Iterator<Item = &KaolinNode<'a>> {
        self.nodes.iter()
    }

    pub fn grow_w(&mut self) {
        self.nodes.iter_mut().for_each(|c| {
            match c.element {
                KaolinElement::Flex(ref mut flex_box) => {
                    flex_box.grow_children_width(c.current_width);
                }
                KaolinElement::Text(ref mut text_element) => {
                    text_element.wrap_text(c.current_width);
                }
            }

            if let KaolinElement::Flex(ref mut flex_box) = c.element {
                flex_box.grow_children_width(c.current_width);
                c.fit_height();
            }
        });
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}
