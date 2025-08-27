pub mod flexbox;
pub mod text;
pub mod traits;

use crate::{
    commands::RenderCommand,
    elements::traits::KaolinElement,
    style::sizing::{PreferredSize, SizingDimensions},
};
use uuid::Uuid;

pub(crate) struct KaolinNode<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    #[allow(dead_code)]
    id: String,
    growable_width: bool,
    growable_height: bool,
    shrinkable: bool,
    element: Box<dyn KaolinElement<Color>>,
    sizing: (SizingDimensions, SizingDimensions),
    current_width: f64,
    current_height: f64,
    x: f64,
    y: f64,
}

impl<Color> KaolinNode<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    pub(crate) fn new(element: impl KaolinElement<Color> + 'static, id: Option<String>) -> Self {
        let id = id.unwrap_or_else(|| Uuid::new_v4().to_string());
        let (width, height) = element.get_sizing_dimensions();
        KaolinNode {
            id,
            growable_width: element.default_growable_width(&width),
            growable_height: element.default_growable_height(&height),
            shrinkable: element.default_shrinkable(&width),
            current_width: element.starting_width(&width),
            current_height: element.starting_height(&height),
            sizing: (width, height),
            element: Box::new(element),
            x: 0.0,
            y: 0.0,
        }
    }

    /// Returns the growth factors for calculations during the growing phase.
    pub fn get_grow_factor(&self) -> (f64, f64) {
        let w_factor = match self.sizing.0.preferred {
            PreferredSize::Grow(factor) => factor.into(),
            _ => 0.0, // Default grow factor if not specified
        };
        let h_factor = match self.sizing.1.preferred {
            PreferredSize::Grow(factor) => factor.into(),
            _ => 0.0, // Default grow factor if not specified
        };
        (w_factor, h_factor)
    }

    /// ### Grows (or shrinks) the width of the element within the given limit.
    /// Returns the amount of growth/shrinkage actually applied.
    ///
    /// The returned value differs from the input value only if the element was
    /// constrained by its minimum or maximum size.
    fn grow_width(&mut self, limit: f64) -> f64 {
        let amount = self.sizing.0.clamped(self.current_width + limit) - self.current_width;
        if self.growable_width && amount > 0.0 {
            self.current_width += amount;
            if amount != limit {
                self.growable_width = false; // No more growth possible
            }
            amount
        } else if self.shrinkable && amount < 0.0 {
            self.current_width += amount;
            if amount != limit {
                self.shrinkable = false; // No more shrinking possible
            }
            amount
        } else {
            // no more change possible at all
            self.growable_width = false;
            self.shrinkable = false;
            0.0
        }
    }

    /// ### Grows the height of the element within the given limit.
    /// Returns the amount of growth actually applied.
    ///
    /// The returned value differs from the input value only if the element was
    /// constrained by its maximum size.
    fn grow_height(&mut self, limit: f64) -> f64 {
        debug_assert!(limit > 0.0, "Height can only grow, never shrink");
        let amount = self.sizing.1.clamped(self.current_height + limit) - self.current_height;
        if self.growable_height && amount > 0.0 {
            self.current_height += amount;
            if amount != limit {
                self.growable_height = false; // No more growth possible
            }
            amount
        } else {
            0.0
        }
    }

    /// Fits the height of the element to its content.
    fn fit_height(&mut self, final_width: f64) {
        let height = self.sizing.1;
        self.current_height = height.clamped(self.element.fit_height_unbound(final_width));
    }

    /// ### Sets the position of the element, and propagates the change to its children.
    /// This gets called after all sizing calculations are complete.
    pub fn set_position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
        self.element
            .propagate_position((x, y), (self.current_width, self.current_height));
    }

    /// Renders the element and its children into a series of rendering commands.
    pub fn render(&self) -> Box<dyn Iterator<Item = RenderCommand<Color>> + '_> {
        self.element
            .render((self.x, self.y), (self.current_width, self.current_height))
    }
}

pub(crate) struct KaolinNodes<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    pub(crate) nodes: Vec<KaolinNode<Color>>,
}

impl<Color> KaolinNodes<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    fn new() -> Self {
        KaolinNodes { nodes: Vec::new() }
    }

    /// Add a new child
    fn push(&mut self, node: KaolinNode<Color>) {
        self.nodes.push(node);
    }

    /// Returns an array of mutable references of all growable children in the horizontal direction.
    fn get_growable_children_w(&mut self) -> Vec<&mut KaolinNode<Color>> {
        self.nodes.iter_mut().filter(|c| c.growable_width).collect()
    }

    /// Returns an array of mutable references of all growable children in the vertical direction.
    fn get_growable_children_h(&mut self) -> Vec<&mut KaolinNode<Color>> {
        self.nodes
            .iter_mut()
            .filter(|c| c.growable_height)
            .collect()
    }

    /// Returns an array of mutable references of all shrinkable children.
    fn get_shrinkable_children(&mut self) -> Vec<&mut KaolinNode<Color>> {
        self.nodes.iter_mut().filter(|c| c.shrinkable).collect()
    }

    /// Looks for the smallest and second smallest widths among the children.
    /// Important to make sure all children grow together.
    fn get_smallest_widths(children: &Vec<&mut KaolinNode<Color>>) -> (f64, f64) {
        let mut smallest = f64::INFINITY;
        let mut second_smallest = f64::INFINITY;
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

    /// Looks for the smallest and second smallest heights among the children.
    /// Important to make sure all children grow together.
    fn get_smallest_heights(children: &Vec<&mut KaolinNode<Color>>) -> (f64, f64) {
        let mut smallest = f64::INFINITY;
        let mut second_smallest = f64::INFINITY;
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

    /// Looks for the biggest and second biggest widths among the children.
    /// Important to make sure all children grow together.
    fn get_biggest_widths(children: &Vec<&mut KaolinNode<Color>>) -> (f64, f64) {
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

    /// Returns the cumulative width of the child nodes.
    fn get_cumulative_width(&self) -> f64 {
        self.nodes.iter().map(|c| c.current_width).sum()
    }

    /// Returns the maximum width of the child nodes.
    fn get_max_width(&self) -> f64 {
        self.nodes
            .iter()
            .fold(0.0, |acc, c| acc.max(c.current_width))
    }

    /// Returns the cumulative height of the child nodes.
    fn get_cumulative_height(&self) -> f64 {
        self.nodes.iter().map(|c| c.current_height).sum()
    }

    /// Returns the maximum height of the child nodes.
    fn get_max_height(&self) -> f64 {
        self.nodes
            .iter()
            .fold(0.0, |acc, c| acc.max(c.current_height))
    }

    /// Returns an iterator over the child nodes.
    fn nodes(&mut self) -> impl Iterator<Item = &mut KaolinNode<Color>> {
        self.nodes.iter_mut()
    }

    /// Propagates the width growth of the element to its children.
    pub fn propagate_width_growth(&mut self) {
        self.nodes().for_each(|c| {
            if let Some(container) = c.element.as_container() {
                container.propagate_width_growth(c.current_width);
            }
            c.fit_height(c.current_width);
        });
    }

    /// Propagates the height growth of the element to its children.
    pub fn do_grow_height(&mut self) {
        self.nodes.iter_mut().for_each(|node| {
            if let Some(container) = node.element.as_container() {
                container.propagate_height_growth(node.current_height);
            }
        });
    }

    pub fn render_nodes(&self) -> Box<dyn Iterator<Item = RenderCommand<Color>> + '_> {
        Box::new(
            self.nodes
                .iter()
                .flat_map(|node: &KaolinNode<Color>| node.render()),
        )
    }

    /// Returns the number of gaps that need to be accounted for.
    pub fn gaps(&self) -> u32 {
        if self.nodes.is_empty() {
            return 0;
        }
        self.nodes.len() as u32 - 1
    }
}
