use alloc::boxed::Box;
use core::ops::Add;

use crate::{
    commands::RenderCommand,
    elements::{
        KaolinNode, KaolinNodes,
        traits::{KaolinContainerElement, KaolinElement},
    },
    style::{
        FlexStyle,
        layout::{Alignment, Direction, Justification},
        sizing::SizingDimensions,
    },
    utils::floats::Float,
};

macro_rules! shrinkable_children {
    ($self:expr) => {
        $self.children.nodes().filter(|c| c.shrinkable)
    };
}

macro_rules! growable_children_w {
    ($self:expr) => {
        $self.children.nodes().filter(|c| c.growable_width)
    };
}

macro_rules! growable_children_h {
    ($self:expr) => {
        $self.children.nodes().filter(|c| c.growable_height)
    };
}

macro_rules! biggest_widths {
    ($children:expr) => {{
        let mut biggest = 0.0;
        let mut second_biggest = 0.0;
        for node in $children {
            if node.current_width > biggest {
                second_biggest = biggest;
                biggest = node.current_width;
            } else if node.current_width > second_biggest && node.current_width < biggest {
                second_biggest = node.current_width;
            }
        }
        (biggest, second_biggest)
    }};
}

macro_rules! smallest_widths {
    ($children:expr) => {{
        let mut smallest = Float::MAX;
        let mut second_smallest = Float::MAX;
        for node in $children {
            if node.current_width < smallest {
                second_smallest = smallest;
                smallest = node.current_width;
            } else if node.current_width < second_smallest && node.current_width > smallest {
                second_smallest = node.current_width;
            }
        }
        (smallest, second_smallest)
    }};
}

macro_rules! smallest_heights {
    ($children:expr) => {{
        let mut smallest = Float::MAX;
        let mut second_smallest = Float::MAX;
        for node in $children {
            if node.current_height < smallest {
                second_smallest = smallest;
                smallest = node.current_height;
            } else if node.current_height < second_smallest && node.current_height > smallest {
                second_smallest = node.current_height;
            }
        }
        (smallest, second_smallest)
    }};
}

macro_rules! modifying {
    ($children:expr, $extreme:expr, width) => {
        $children.filter(|c| c.current_width == $extreme)
    };
    ($children:expr, $extreme:expr, height) => {
        $children.filter(|c| c.current_height == $extreme)
    };
}

pub(crate) struct FlexBox<'frame, Color, CustomData>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    style: FlexStyle<Color>,
    pub(crate) children: KaolinNodes<'frame, Color, CustomData>,
    pub(crate) inherited_color: Option<Color>,
}

impl<'frame, Color, CustomData> FlexBox<'frame, Color, CustomData>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    pub fn new(style: FlexStyle<Color>) -> Self {
        FlexBox {
            style,
            children: KaolinNodes::new(),
            inherited_color: None,
        }
    }

    fn get_cumulative_gaps(&self) -> Float {
        self.children.gaps() as Float * self.style.layout.gap
    }

    /// Fits the width of the flex container to its children, returning the new width.
    fn fit_width_to_children(&self) -> Float {
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
    pub(crate) fn grow_children_width(&mut self, current_width: Float) {
        if self.style.has_horizontal_layout() {
            let cum_width = self.children.get_cumulative_width(); // lmao
            let mut remaining =
                current_width - self.style.padding.x() - cum_width - self.get_cumulative_gaps();

            if remaining < 0.0 {
                // Shrinking
                while remaining < -1.0 && shrinkable_children!(self).count() > 0 {
                    let mut total_change = 0.0;
                    //  total factor for dividing the available space
                    let (extreme, second_extreme) = biggest_widths!(shrinkable_children!(self));
                    let total_factor =
                        modifying!(shrinkable_children!(self), extreme, width).count() as Float;
                    if total_factor <= 0.0 {
                        break; // avoid infinite loops, means no progress can be made
                    }

                    // calculate the base change amount for each child
                    let change_amount = remaining.max(second_extreme - extreme) / total_factor;
                    for child in modifying!(shrinkable_children!(self), extreme, width) {
                        // grow the child to the new width
                        let change = child.grow_width(change_amount);
                        total_change += change; // keep track of the total change
                    }

                    // remove the amount we changed from the remaining space
                    remaining -= total_change;
                }
            } else {
                // Growing
                while remaining > 1.0 && growable_children_w!(self).count() > 0 {
                    let mut total_change = 0.0;
                    let (extreme, second_extreme) = smallest_widths!(growable_children_w!(self));

                    //  total factor for dividing the available space
                    let total_factor = modifying!(growable_children_w!(self), extreme, width)
                        .map(|c| c.get_grow_factor().0)
                        .sum::<Float>();

                    if total_factor <= 0.0 {
                        break; // avoid infinite loops, means no progress can be made
                    }

                    // calculate the base change amount for each child
                    let change_amount = remaining.min(second_extreme - extreme) / total_factor;
                    for child in modifying!(growable_children_w!(self), extreme, width) {
                        // grow the child to the new width
                        let change = child.grow_width(change_amount * child.get_grow_factor().0);
                        total_change += change; // keep track of the total change
                    }

                    // remove the amount we changed from the remaining space
                    remaining -= total_change;
                }
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
    pub(crate) fn grow_children_height(&mut self, current_height: Float) {
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

            while remaining > 0.0 && growable_children_h!(self).count() > 0 {
                let mut total_growth = 0.0;
                let (extreme, second_extreme) = smallest_heights!(growable_children_h!(self));
                let total_factor = modifying!(growable_children_h!(self), extreme, height)
                    .map(|c| c.get_grow_factor().1)
                    .sum::<Float>();
                if total_factor > 0.0 {
                    let grow_amount = remaining.min(second_extreme - extreme) / total_factor;
                    for child in modifying!(growable_children_h!(self), extreme, height) {
                        let factor = child.get_grow_factor().1;
                        let grow = child.grow_height(grow_amount * factor);
                        total_growth += grow;
                    }
                } else {
                    break; // avoid infinite loop
                }
                remaining -= total_growth;
            }
        }
        self.children.do_grow_height();
    }

    /// Positions the child elements within the flex container.
    /// Called after all sizing calculations are complete.
    pub(crate) fn position_children(&mut self, offsets: (Float, Float, Float, Float)) {
        let (left, right, top, bottom) = offsets;

        let (tot_main_dimension, tot_cross_dimension) =
            self.style.switch_axis((right - left, bottom - top));

        // number of gaps between children
        let n_gaps = self.children.gaps() as Float;
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
}

impl<'frame, Color, CustomData> KaolinElement<'frame, Color, CustomData>
    for FlexBox<'frame, Color, CustomData>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    fn inherit_color(&mut self, inherited_color: Color) {
        self.inherited_color = Some(self.style.color.unwrap_or(inherited_color));
    }

    fn get_sizing_dimensions(&self) -> (SizingDimensions, SizingDimensions) {
        let width = SizingDimensions::from(self.style.sizing.width);
        let height = SizingDimensions::from(self.style.sizing.height);
        (width, height)
    }

    fn fit_height_unbound(&mut self, _final_width: Float) -> Float {
        if self.style.has_horizontal_layout() {
            self.children.get_max_height().add(self.style.padding.y())
        } else {
            self.children.get_cumulative_height()
                + self.get_cumulative_gaps()
                + self.style.padding.y()
        }
    }

    fn starting_width(&self, sizing: &SizingDimensions) -> Float {
        sizing.clamped(self.fit_width_to_children())
    }

    fn propagate_position(&mut self, offsets: (Float, Float), size: (Float, Float)) {
        self.position_children((offsets.0, offsets.0 + size.0, offsets.1, offsets.1 + size.1));
    }

    fn render(
        &self,
        offsets: (Float, Float),
        size: (Float, Float),
    ) -> Box<dyn Iterator<Item = RenderCommand<Color, CustomData>> + '_> {
        let self_command = RenderCommand::DrawRectangle {
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

    fn as_container(
        &mut self,
    ) -> Option<&mut dyn KaolinContainerElement<'frame, Color, CustomData>> {
        Some(self)
    }
}

impl<'frame, Color, CustomData> KaolinContainerElement<'frame, Color, CustomData>
    for FlexBox<'frame, Color, CustomData>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    fn add_child(&mut self, child: KaolinNode<'frame, Color, CustomData>) {
        self.children.push(child);
    }

    fn propagate_width_growth(&mut self, parent_width: Float) {
        self.grow_children_width(parent_width);
    }

    fn propagate_height_growth(&mut self, parent_height: Float) {
        self.grow_children_height(parent_height);
    }
}
