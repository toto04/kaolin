use alloc::boxed::Box;

use crate::elements::KaolinNode;
use crate::elements::RenderCommand;
use crate::style::sizing::PreferredSize;
use crate::style::sizing::SizingDimensions;

/// This trait represents a generic UI element in the Kaolin layout system.
///
/// You can create your own elements by implementing this trait, and providing the necessary
/// methods to handle layout, rendering, and interaction.
pub trait KaolinElement<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    /// This function gets called to initialize a [`KaolinNode`] with the element's sizing information.
    /// See [`SizingDimensions`] for more details.
    ///
    /// This function should return sizing behavior in the form (width, height)
    fn get_sizing_dimensions(&self) -> (SizingDimensions, SizingDimensions);

    /// This function gets called to render the element, and should return an iterator
    /// over the render commands for the element (and its children if any).
    fn render(
        &self,
        offsets: (f64, f64),
        size: (f64, f64),
    ) -> Box<dyn Iterator<Item = RenderCommand<Color>> + '_>;

    /// whether or not the element starts as growable in width, defaults to
    /// [`SizingDimensions::is_growable`]
    fn default_growable_width(&self, sizing: &SizingDimensions) -> bool {
        sizing.is_growable()
    }
    /// whether or not the element starts as shrinkable, defaults to
    /// [`SizingDimensions::is_shrinkable`]
    fn default_shrinkable(&self, sizing: &SizingDimensions) -> bool {
        sizing.is_shrinkable()
    }
    /// whether or not the element starts as growable in height, defaults to
    /// [`SizingDimensions::is_growable`]
    fn default_growable_height(&self, sizing: &SizingDimensions) -> bool {
        sizing.is_growable()
    }
    /// Returns the initial width of the element based on its sizing configuration.
    /// By default, the preferred size is used if available, otherwise the minimum size is used.
    fn starting_width(&self, sizing: &SizingDimensions) -> f64 {
        match sizing.preferred {
            PreferredSize::Fixed(size) => size.into(),
            _ => sizing.min.into(),
        }
    }
    /// Returns the initial height of the element based on its sizing configuration.
    /// By default, the preferred size is used if available, otherwise the minimum size is used.
    fn starting_height(&self, sizing: &SizingDimensions) -> f64 {
        match sizing.preferred {
            PreferredSize::Fixed(size) => size.into(),
            _ => sizing.min.into(),
        }
    }

    /// This function should be overridden to give a absolute lower bound for the
    /// node's height, based on the element's content and final width.
    /// This function can also be used to handle side-effects of width finalization
    ///
    /// This function should not consider any bounds (minimum / maximum sizing)
    /// as the value will always be clamped to the element's sizing configuration.
    #[allow(unused_variables)]
    fn fit_height_unbound(&mut self, final_width: f64) -> f64 {
        0.0
    }

    /// This function should be overridden when position changes needs to be
    /// propagated to the element contents. The offsets and size are the element's
    /// final values, which should be interpreted as absolute.
    #[allow(unused_variables)]
    fn propagate_position(&mut self, offsets: (f64, f64), size: (f64, f64)) {}

    /// This function can be overridden to inherit the color property from the parent element.
    /// Useful for elements like text, that should remain consistent with the parent if not otherwised specified.
    #[allow(unused_variables)]
    fn inherit_color(&mut self, inherited_color: Color) {}

    /// Tries to downcast the element to a container type.
    /// Must be implemented by each element, returning either `None` or `Some(self)`
    /// based on whether the element should behave like a container or not.
    ///
    /// To be able to return `Some(self)`, the element must implement the [`KaolinContainerElement`] trait.
    fn as_container(&mut self) -> Option<&mut dyn KaolinContainerElement<Color>>;
}

/// This trait represents an element which can contain other elements
///
/// Structs should implement this if they want to act as containers for other generic elements.
///
/// For this to work, the element must also correctly implement [`KaolinElement::as_container`]
///
/// ```ignore
/// impl<Color> KaolinElement<Color> for MyContainerElement<Color> {
///     fn as_container(&mut self) -> Option<&mut dyn KaolinContainerElement<Color>> {
///         Some(self)
///     }
///     ...
/// }
/// ```
pub trait KaolinContainerElement<Color>: KaolinElement<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    /// Adds a child node to the container.
    fn add_child(&mut self, child: KaolinNode<Color>);
    /// Propagates width growth to the container's children.
    fn propagate_width_growth(&mut self, parent_width: f64);
    /// Propagates height growth to the container's children.
    fn propagate_height_growth(&mut self, parent_height: f64);
}
