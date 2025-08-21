use std::ops::Add;

use crate::{
    commands::{RenderCommand, RenderCommands},
    elements::{KaolinElement, KaolinNode, KaolinNodes},
    style::{
        FlexStyle,
        layout::{Alignment, Direction, Justification},
    },
};

pub struct FlexBox<'frame, Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    pub style: FlexStyle<Color>,
    pub(crate) children: KaolinNodes<'frame, Color>,
}

impl<'frame, Color> FlexBox<'frame, Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    pub fn new(style: FlexStyle<Color>) -> Self {
        FlexBox {
            style,
            children: KaolinNodes::new(),
        }
    }

    /// Fits the width of the flex container to its children, returning the new width.
    pub fn fit_width_to_children(&self) -> f64 {
        match self.style.layout.direction {
            Direction::LeftToRight | Direction::RightToLeft => {
                let gaps = self.children.gaps();
                self.children.get_cumulative_width()
                    + self.style.padding.x()
                    + (gaps as f64) * self.style.layout.gap
            }
            Direction::TopToBottom | Direction::BottomToTop => {
                self.children.get_max_width() + self.style.padding.x()
            }
        }
    }

    /// Grows the width of all child elements to fit the container.
    pub fn grow_children_width(&mut self, current_width: f64) {
        match self.style.layout.direction {
            Direction::LeftToRight | Direction::RightToLeft => {
                let gaps = self.children.gaps();
                let cum_width = self.children.get_cumulative_width(); // lmao
                let mut remaining = current_width
                    - self.style.padding.x()
                    - cum_width
                    - (gaps as f64) * self.style.layout.gap;

                if remaining > 0.0 {
                    // grow
                    let mut growable_children = self.children.get_growable_children_w();
                    while remaining > 0.0 && !growable_children.is_empty() {
                        let mut total_growth = 0.0;
                        let (smallest, second_smallest) =
                            KaolinNodes::get_smallest_widths(&growable_children);

                        let growing = growable_children
                            .iter_mut()
                            .filter(|c| c.current_width == smallest)
                            .collect::<Vec<_>>();

                        let total_factor =
                            growing.iter().map(|c| c.get_grow_factor().0).sum::<f64>();
                        if total_factor > 0.0 {
                            let grow_amount =
                                remaining.min(second_smallest - smallest) / total_factor;
                            for child in growing {
                                let factor = child.get_grow_factor().0;
                                let growth = child.grow_width(grow_amount * factor);
                                total_growth += growth;
                            }
                        } else {
                            break;
                        }
                        remaining -= total_growth;
                        growable_children.retain(|c| c.growable_width);
                    }
                } else if remaining < 0.0 {
                    // shrink
                    let mut shrinkable_children = self.children.get_shrinkable_children();
                    while remaining < 0.0 && !shrinkable_children.is_empty() {
                        let mut total_shrink = 0.0;
                        let (biggest, second_biggest) =
                            KaolinNodes::get_biggest_widths(&shrinkable_children);

                        let shrinking = shrinkable_children
                            .iter_mut()
                            .filter(|c| c.current_width == biggest)
                            .collect::<Vec<_>>();

                        let len = shrinking.len() as f64;
                        let shrink_amount = -remaining.abs().min(biggest - second_biggest) / len;
                        for child in shrinking {
                            let shrink = child.grow_width(shrink_amount);
                            total_shrink += shrink;
                        }

                        remaining -= total_shrink;
                        shrinkable_children.retain(|c| c.shrinkable);
                    }
                }
            }
            Direction::TopToBottom | Direction::BottomToTop => {
                self.children.nodes().for_each(|child| {
                    let remaining = current_width - child.current_width - self.style.padding.x();
                    if (remaining > 0.0 && child.growable_width)
                        || (remaining < 0.0 && child.shrinkable)
                    {
                        child.grow_width(remaining);
                    }
                    child.growable_width = false; // Once grown, they can't grow anymore
                });
            }
        }
        self.children.grow_w();
    }

    /// Fits the height of the flex container to its children, returning the new height.
    pub fn fit_height_to_children(&mut self, mut current_height: f64, max: f64) -> f64 {
        // self.children.fit_heights();
        let len = self.children.len() as f64;

        match self.style.layout.direction {
            Direction::LeftToRight | Direction::RightToLeft => {
                current_height = self
                    .children
                    .get_max_height()
                    .add(self.style.padding.y())
                    .min(max);
            }

            Direction::TopToBottom | Direction::BottomToTop => {
                current_height += self.children.get_cumulative_height();
                current_height += (len - 1.0) * self.style.layout.gap;
                current_height += self.style.padding.y();
                current_height = current_height.min(max);
            }
        }
        current_height
    }

    /// Grows the height of all child elements to fit the container.
    pub fn grow_children_height(&mut self, current_height: f64) {
        match self.style.layout.direction {
            Direction::LeftToRight | Direction::RightToLeft => {
                self.children
                    .get_growable_children_h()
                    .iter_mut()
                    .for_each(|child| {
                        if child.growable_height {
                            child.grow_height(current_height - child.current_height);
                        }
                    });
            }
            Direction::TopToBottom | Direction::BottomToTop => {
                let cum_height = self.children.get_cumulative_height(); // lmao
                let mut remaining = current_height - self.style.padding.y() - cum_height;
                let mut growable_children = self.children.get_growable_children_h();

                while remaining > 0.0 && !growable_children.is_empty() {
                    let mut total_grow = 0.0;
                    let (smallest, second_smallest) =
                        KaolinNodes::get_smallest_heights(&growable_children);

                    let growing = growable_children
                        .iter_mut()
                        .filter(|c| c.current_width == smallest)
                        .collect::<Vec<_>>();

                    let total_factor = growing.iter().map(|c| c.get_grow_factor().1).sum::<f64>();
                    if total_factor > 0.0 {
                        let grow_amount = remaining.min(second_smallest - smallest) / total_factor;
                        for child in growing {
                            let factor = child.get_grow_factor().1;
                            let grow = child.grow_height(grow_amount * factor);
                            total_grow += grow;
                        }
                    }
                    remaining -= total_grow;
                    growable_children.retain(|c| c.growable_height);
                }
            }
        }
        self.children.grow_h();
    }

    /// Positions the child elements within the flex container.
    /// Called after all sizing calculations are complete.
    pub fn position_children(&mut self, offsets: (f64, f64, f64, f64)) {
        let (left, right, top, bottom) = offsets;
        match self.style.layout.direction {
            Direction::LeftToRight | Direction::RightToLeft => {
                let cum_width = self.children.get_cumulative_width()
                    + self.children.gaps() as f64 * self.style.layout.gap;
                let mut x = match self.style.layout.justification {
                    Justification::Start
                    | Justification::SpaceBetween
                    | Justification::SpaceAround => left + self.style.padding.left,
                    Justification::End => right - left - cum_width - self.style.padding.right,
                    Justification::Center => {
                        (right - left - self.style.padding.x() - cum_width) / 2.0
                            + left
                            + self.style.padding.left
                    }
                };

                for child in &mut self.children.nodes {
                    let y = match self.style.layout.alignment {
                        Alignment::Start | Alignment::Stretch => top + self.style.padding.top,
                        Alignment::Center => {
                            (bottom - top - self.style.padding.y() - child.current_height) / 2.0
                                + top
                                + self.style.padding.top
                        }
                        Alignment::End => bottom - self.style.padding.bottom - child.current_height,
                    };
                    child.set_position(x, y);
                    x += child.current_width + self.style.layout.gap;
                }
            }
            Direction::TopToBottom | Direction::BottomToTop => {
                let cum_height = self.children.get_cumulative_height();
                let mut y = match self.style.layout.justification {
                    Justification::Start
                    | Justification::SpaceBetween
                    | Justification::SpaceAround => top + self.style.padding.top,
                    Justification::End => bottom - top - cum_height - self.style.padding.bottom,
                    Justification::Center => {
                        (bottom - top - self.style.padding.y() - cum_height) / 2.0
                            + top
                            + self.style.padding.top
                    }
                };

                for child in &mut self.children.nodes {
                    let x = match self.style.layout.alignment {
                        Alignment::Start | Alignment::Stretch => left + self.style.padding.left,
                        Alignment::Center => {
                            (right - left - self.style.padding.x() - child.current_width) / 2.0
                                + left
                                + self.style.padding.left
                        }
                        Alignment::End => right - self.style.padding.right - child.current_width,
                    };
                    child.set_position(x, y);
                    y += child.current_height + self.style.layout.gap;
                }
            }
        }
    }

    /// Adds a new child to the flex container.
    pub fn add_child(&mut self, child: KaolinElement<'frame, Color>) {
        self.children.push(KaolinNode::new(child, None));
    }

    /// Creates an iterator representing all render commands for itself and its children in order.
    pub fn render(
        &self,
        self_command: RenderCommand<Color>,
        inherited_color: Color,
    ) -> impl Iterator<Item = RenderCommand<Color>> {
        std::iter::once(self_command).chain(
            self.children
                .render_nodes(self.style.color.unwrap_or(inherited_color)),
        )
    }

    /// Consumes the root flex container and all of its children in the element
    /// tree, producing a series of rendering commands.
    pub fn conclude(self) -> RenderCommands<Color> {
        RenderCommands::new(self)
    }
}
