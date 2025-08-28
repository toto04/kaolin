use core::{cmp::min_by, ops::Add};

use alloc::{boxed::Box, string::ToString, vec::Vec};

use crate::{
    commands::{RenderCommand, RenderCommands},
    elements::{
        KaolinNode, KaolinNodes,
        traits::{KaolinContainerElement, KaolinElement},
    },
    style::{
        FlexStyle,
        layout::{Alignment, Direction, Justification},
        sizing::SizingDimensions,
    },
};

pub(crate) struct FlexBox<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    style: FlexStyle<Color>,
    pub(crate) children: KaolinNodes<Color>,
    pub(crate) inherited_color: Option<Color>,
}

impl<Color> FlexBox<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color> + 'static,
{
    pub fn new(style: FlexStyle<Color>) -> Self {
        FlexBox {
            style,
            children: KaolinNodes::new(),
            inherited_color: None,
        }
    }

    fn get_cumulative_gaps(&self) -> f64 {
        self.children.gaps() as f64 * self.style.layout.gap
    }

    /// Fits the width of the flex container to its children, returning the new width.
    fn fit_width_to_children(&self) -> f64 {
        match self.style.layout.direction {
            Direction::LeftToRight | Direction::RightToLeft => {
                self.children.get_cumulative_width()
                    + self.style.padding.x()
                    + self.get_cumulative_gaps()
            }
            Direction::TopToBottom | Direction::BottomToTop => {
                self.children.get_max_width() + self.style.padding.x()
            }
        }
    }

    /// Grows the width of all child elements to fit the container.
    pub(crate) fn grow_children_width(&mut self, current_width: f64) {
        if self.style.has_horizontal_layout() {
            let cum_width = self.children.get_cumulative_width(); // lmao
            let mut remaining =
                current_width - self.style.padding.x() - cum_width - self.get_cumulative_gaps();

            let shrinking = remaining < 0.0;

            // create a list of children that will be subject to shrinking/growing
            let mut modifiable_children = if shrinking {
                self.children.get_shrinkable_children()
            } else {
                self.children.get_growable_children_w()
            };

            while remaining.abs() > 0.0 && !modifiable_children.is_empty() {
                let mut total_change = 0.0;
                // Get the current extreme widths
                // the extreme is the starting point for the growth/shrinking, the second extreme is the stopping point for this iteration
                let (extreme, second_extreme) = if shrinking {
                    KaolinNodes::get_biggest_widths(&modifiable_children)
                } else {
                    KaolinNodes::get_smallest_widths(&modifiable_children)
                };

                // get the list of children that have the current extreme width
                let currently_modifying = modifiable_children
                    .iter_mut()
                    .filter(|c| c.current_width == extreme)
                    .collect::<Vec<_>>();

                //  total factor for dividing the available space
                let total_factor = if shrinking {
                    currently_modifying.len() as f64
                } else {
                    currently_modifying
                        .iter()
                        .map(|c| c.get_grow_factor().0)
                        .sum::<f64>()
                };

                if total_factor <= 0.0 {
                    break; // avoid infinite loops, means no progress can be made
                }

                // calculate the base change amount for each child
                let change_amount = min_by(remaining, second_extreme - extreme, |a, b| {
                    a.abs()
                        .partial_cmp(&b.abs())
                        .unwrap_or_else(|| panic!("wtf"))
                }) / total_factor;

                for child in currently_modifying {
                    // how much of the base change amount should this child get?
                    let factor = if shrinking {
                        1.0
                    } else {
                        child.get_grow_factor().0
                    };
                    // grow the child to the new width
                    let change = child.grow_width(change_amount * factor);
                    total_change += change; // keep track of the total change
                }

                // remove the amount we changed from the remaining space
                remaining -= total_change;
                // retain only the children that can still be modified
                modifiable_children.retain(|c| {
                    if shrinking {
                        c.shrinkable
                    } else {
                        c.growable_width
                    }
                });
            }
        } else {
            // in cross axis, the growth is done individually for each child instead of sequentially
            self.children.nodes().for_each(|child| {
                // each has its own remaining space
                let remaining = current_width - child.current_width - self.style.padding.x();
                if (remaining > 0.0 && child.growable_width)
                    || (remaining < 0.0 && child.shrinkable)
                {
                    child.grow_width(remaining);
                }
                child.growable_width = false; // Once grown, they can't grow anymore
            });
        }
        self.children.propagate_width_growth();
    }

    /// Grows the height of all child elements to fit the container.
    pub(crate) fn grow_children_height(&mut self, current_height: f64) {
        if self.style.has_horizontal_layout() {
            self.children.nodes().for_each(|child| {
                let remaining = current_height - child.current_height - self.style.padding.y();
                if remaining > 0.0 && child.growable_height {
                    child.grow_height(remaining);
                }
                child.growable_height = false; // Once grown, they can't grow anymore
            });
        } else {
            let cum_height = self.children.get_cumulative_height(); // lmao
            let mut remaining =
                current_height - self.style.padding.y() - cum_height - self.get_cumulative_gaps();
            let mut growable_children = self.children.get_growable_children_h();

            while remaining > 0.0 && !growable_children.is_empty() {
                let mut total_growth = 0.0;
                let (smallest, second_smallest) =
                    KaolinNodes::get_smallest_heights(&growable_children);

                let growing = growable_children
                    .iter_mut()
                    .filter(|c| c.current_height == smallest)
                    .collect::<Vec<_>>();

                let total_factor = growing.iter().map(|c| c.get_grow_factor().1).sum::<f64>();
                if total_factor > 0.0 {
                    let grow_amount = remaining.min(second_smallest - smallest) / total_factor;
                    for child in growing {
                        let factor = child.get_grow_factor().1;
                        let grow = child.grow_height(grow_amount * factor);
                        total_growth += grow;
                    }
                } else {
                    break; // avoid infinite loop
                }
                remaining -= total_growth;
                growable_children.retain(|c| c.growable_height);
            }
        }
        self.children.do_grow_height();
    }

    /// Positions the child elements within the flex container.
    /// Called after all sizing calculations are complete.
    pub(crate) fn position_children(&mut self, offsets: (f64, f64, f64, f64)) {
        let (left, right, top, bottom) = offsets;

        let (tot_main_dimension, tot_cross_dimension) =
            self.style.switch_axis((right - left, bottom - top));

        // number of gaps between children
        let n_gaps = self.children.gaps() as f64;
        // cumulative space occupied by the children in the main axis
        let cum_dimension = if self.style.has_horizontal_layout() {
            self.children.get_cumulative_width()
        } else {
            self.children.get_cumulative_height()
        };

        // total usable empty space in the main axis
        let empty_dimension = tot_main_dimension - cum_dimension;

        // the actual distance the elements need to be apart in the main axis
        let gap = match self.style.layout.justification {
            Justification::SpaceBetween => empty_dimension / n_gaps,
            Justification::SpaceAround => empty_dimension / (n_gaps + 2.0),
            _ => 0.0,
        }
        .max(self.style.layout.gap); // never go below the set gap

        let leftover_dimension = empty_dimension - gap * n_gaps; // unused space by the children in the main axis

        // first drawable point for the children in both dimensions
        let (main_starting_offset, cross_starting_offset) = self
            .style
            .switch_axis((left + self.style.padding.left, top + self.style.padding.top));

        // last drawable point for the children in both dimensions
        let (main_ending_offset, cross_ending_offset) = self.style.switch_axis((
            right - self.style.padding.right,
            bottom - self.style.padding.bottom,
        ));

        // cumulative padding in both dimensions
        let (main_pad, cross_pad) = self
            .style
            .switch_axis((self.style.padding.x(), self.style.padding.y()));

        // main axis position
        let mut main_axis = match self.style.layout.justification {
            Justification::Start | Justification::SpaceBetween => {
                main_starting_offset // left offset and padding
            }
            Justification::SpaceAround => main_starting_offset + gap, // space around adds the gap to the outside
            Justification::End => main_ending_offset - cum_dimension - gap * n_gaps, // whatever is missing from the total, including right padding
            Justification::Center => main_starting_offset + (leftover_dimension - main_pad) / 2.0,
        };

        // empty space for each child in the cross dimension
        let usable_cross_dimension = tot_cross_dimension - cross_pad;

        for child in self.children.nodes() {
            let (main_child_dimension, cross_child_dimension) = self
                .style
                .switch_axis((child.current_width, child.current_height));

            let cross_axis = match self.style.layout.alignment {
                Alignment::Start | Alignment::Stretch => cross_starting_offset,
                Alignment::Center => {
                    cross_starting_offset + (usable_cross_dimension - cross_child_dimension) / 2.0
                }
                Alignment::End => cross_ending_offset - cross_child_dimension,
            };
            if self.style.has_horizontal_layout() {
                child.set_position(main_axis, cross_axis);
            } else {
                child.set_position(cross_axis, main_axis);
            }
            main_axis += main_child_dimension + gap;
        }
    }

    /// Consumes the root flex container and all of its children in the element
    /// tree, producing a series of rendering commands.
    pub(crate) fn conclude(self) -> RenderCommands<Color> {
        RenderCommands::new(self)
    }
}

impl<Color> KaolinElement<Color> for FlexBox<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color> + 'static,
{
    fn inherit_color(&mut self, inherited_color: Color) {
        self.inherited_color = Some(self.style.color.unwrap_or(inherited_color));
    }

    fn get_sizing_dimensions(&self) -> (SizingDimensions, SizingDimensions) {
        let width = SizingDimensions::from(self.style.sizing.width);
        let height = SizingDimensions::from(self.style.sizing.height);
        (width, height)
    }

    fn fit_height_unbound(&mut self, _final_width: f64) -> f64 {
        if self.style.has_horizontal_layout() {
            self.children.get_max_height().add(self.style.padding.y())
        } else {
            self.children.get_cumulative_height()
                + self.get_cumulative_gaps()
                + self.style.padding.y()
        }
    }

    fn starting_width(&self, sizing: &SizingDimensions) -> f64 {
        sizing.clamped(self.fit_width_to_children())
    }

    fn propagate_position(&mut self, offsets: (f64, f64), size: (f64, f64)) {
        self.position_children((offsets.0, offsets.0 + size.0, offsets.1, offsets.1 + size.1));
    }

    fn render(
        &self,
        offsets: (f64, f64),
        size: (f64, f64),
    ) -> Box<dyn Iterator<Item = RenderCommand<Color>> + '_> {
        let self_command = RenderCommand::DrawRectangle {
            id: "".to_string(),
            x: offsets.0,
            y: offsets.1,
            width: size.0,
            height: size.1,
            color: self
                .style
                .background_color
                .unwrap_or(Color::default_background_color()),
            corner_radius: self.style.corner_radius,
            border: self.style.border,
        };
        Box::new(core::iter::once(self_command).chain(self.children.render_nodes()))
    }

    fn as_container(&mut self) -> Option<&mut dyn KaolinContainerElement<Color>> {
        Some(self)
    }
}

impl<Color> KaolinContainerElement<Color> for FlexBox<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color> + 'static,
{
    fn add_child(&mut self, child: KaolinNode<Color>) {
        self.children.push(child);
    }

    fn propagate_width_growth(&mut self, parent_width: f64) {
        self.grow_children_width(parent_width);
    }

    fn propagate_height_growth(&mut self, parent_height: f64) {
        self.grow_children_height(parent_height);
    }
}
