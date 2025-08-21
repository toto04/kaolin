/// Definitions of the rendering commands that will be used to draw the UI elements.
pub mod commands;

/// Internal representation of the layout elements.
mod elements;

/// # Kaolin Layout
///
/// This module contains the logic for the layout computation.
///
/// ## Layout calculations order:
/// 1. ***Fit the width*** --> when creating the node (right after the children have already been added)
/// 2. ***Grow & shrink the width*** --> before the children, top down when all the tree is already built
/// 3. ***Wrap the text*** --> as we grow, when dealing with text (same time as step 2)
/// 4. ***Fit the height*** --> when all children have grown in width, fit the height before returning
/// 5. ***Grow & shrink the height*** --> last step after all width has grown and height has been fit, top-down
/// 6. ***Position and align all elements***
///
/// > -- Call the drawing function \
/// >  | -- Create children \
/// >  | -- Fit to width \
/// > -- Grow width \
/// >  | -- Grow children width \
/// >  | -- Fit to height \
/// > -- Grow height \
/// > -- Position and align all elements
pub mod kaolin;

/// Renderers, the things that get the commands and draw stuff on the screen.
pub mod renderers;

/// Contains types and traits for styling UI elements.
///
/// This module provides definitions for both Flex Boxes with [`FlexStyle`] and text configuration with [`TextStyle`].
pub mod style;

pub use kaolin::Kaolin;
