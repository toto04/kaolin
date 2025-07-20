use std::ops::Add;

use crate::{
    elements::{KaolinElement, KaolinNode, KaolinNodes},
    style::{FlexStyle, layout::Direction},
};

pub struct FlexBox<'a> {
    pub style: FlexStyle,
    children: KaolinNodes<'a>,
}

impl<'a> FlexBox<'a> {
    pub fn new(style: FlexStyle) -> Self {
        FlexBox {
            style,
            children: KaolinNodes::new(),
        }
    }

    pub fn fit_to_width(&self) -> f32 {
        match self.style.layout.direction {
            Direction::LeftToRight | Direction::RightToLeft => {
                let gaps = self.children.len() - 1;
                self.children.get_cumulative_width()
                    + self.style.padding.x()
                    + (gaps as f32) * self.style.layout.gap
            }
            Direction::TopToBottom | Direction::BottomToTop => {
                self.children.get_max_width() + self.style.padding.x()
            }
        }
    }

    pub fn grow_children_width(&mut self, current_width: f32) {
        match self.style.layout.direction {
            Direction::LeftToRight | Direction::RightToLeft => {
                let cum_width = self.children.get_cumulative_width(); // lmao
                let mut growable_children = self.children.get_growable_children_w();
                let mut remaining = current_width - self.style.padding.x() - cum_width;

                while remaining > 0.0 && !growable_children.is_empty() {
                    let mut total_grow = 0.0;
                    let (smallest, second_smallest) =
                        KaolinNodes::get_smallest_widths(&growable_children);

                    let growing = growable_children
                        .iter_mut()
                        .filter(|c| c.current_width == smallest)
                        .collect::<Vec<_>>();

                    let total_factor = growing.iter().map(|c| c.get_grow_factor().0).sum::<f32>();

                    if total_factor > 0.0 {
                        let grow_amount = remaining.min(second_smallest - smallest) / total_factor;
                        for child in growing {
                            let factor = child.get_grow_factor().0;
                            let grow = child.grow_width(grow_amount * factor);
                            total_grow += grow;
                        }
                    }

                    remaining -= total_grow;

                    growable_children.retain(|c| c.growable_width);
                }
            }
            Direction::TopToBottom | Direction::BottomToTop => {
                self.children
                    .get_growable_children_w()
                    .iter_mut()
                    .for_each(|child| {
                        let remaining =
                            current_width - child.current_width - self.style.padding.x();
                        child.grow_width(remaining);
                        child.growable_width = false; // Once grown, they can't grow anymore
                    });
            }
        }
        self.children.grow_w();
    }

    pub fn fit_height_to_children(&mut self, mut current_height: f32, max: f32) -> f32 {
        // self.children.fit_heights();
        let len = self.children.len() as f32;

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

    pub fn add_child(&mut self, child: KaolinElement<'a>) {
        self.children.push(KaolinNode::new(child, None));
    }
}
