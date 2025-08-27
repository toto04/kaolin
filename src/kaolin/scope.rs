use crate::{
    elements::{
        KaolinNode,
        flexbox::FlexBox,
        text::TextElement,
        traits::{KaolinContainerElement, KaolinElement},
    },
    kaolin::MeasureTextFnRef,
    style::{FlexStyle, TextStyle},
};

pub struct KaolinScope<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    flex: FlexBox<Color>,
    measure_text: MeasureTextFnRef<Color>,
}

impl<Color> KaolinScope<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color> + 'static,
{
    /// Creates a new root scope, this is where the layout tree begins.
    /// ### This should not be used externally, if you are looking for a way to create a new child element with its own scope, see [KaolinScope::with].
    ///
    /// A new scope is also created for each child flex container, iteratively
    /// allowing for nested layouts.
    pub(super) fn new(flex: FlexBox<Color>, measure_text: MeasureTextFnRef<Color>) -> Self {
        KaolinScope { flex, measure_text }
    }

    /// Conclude the current scope and return the flex container.
    /// This function is called internally when the component tree definition is
    /// completed, allowing the root element to access the finalized layout, consuming
    /// the scope in the process.
    pub(super) fn conclude(self) -> FlexBox<Color> {
        self.flex
    }

    /// ### Create a child container within this element
    ///
    /// This function allows you to set the style for the child flex container,
    /// along with a function which takes a new scope as an argument, allowing you
    /// to recursively set the contents of the child container.
    ///
    /// The scope gets also passed through and returned for chaining sibling
    /// elements.
    ///
    /// Example:
    /// ```ignore
    /// k.with(FlexStyle::new(), |k| {
    ///     k.text("Hello, world!", TextStyle::new()) // inside the new child
    /// })
    /// .text("Hello from the parent!", TextStyle::new()) // inside the parent, after the child
    /// ```
    pub fn with(
        mut self,
        style: FlexStyle<Color>,
        contents: impl Fn(KaolinScope<Color>) -> KaolinScope<Color>,
    ) -> Self {
        let mut child_flex = FlexBox::new(style);
        let color = style
            .color
            .or(self.flex.inherited_color)
            .unwrap_or(Color::default_foreground_color());
        child_flex.inherit_color(color);
        let child_scope = KaolinScope::new(child_flex, self.measure_text.clone());
        let modified_scope = contents(child_scope);
        let child_flex = modified_scope.conclude();
        self.flex.add_child(KaolinNode::new(child_flex, None));
        self
    }

    /// ### Add a text element to the current scope
    ///
    /// This function allows you to add a text element to the current scope.
    /// The text element will be added as a child of the current flex container for which the scope was created.
    ///
    /// The scope gets also passed through and returned for chaining sibling
    /// elements.
    ///
    /// Example:
    /// ```ignore
    /// k.text("Hello, world!", TextStyle::new()) // new text element inside the flex container
    /// ```
    pub fn text(mut self, content: &str, style: TextStyle<Color>) -> Self {
        let mut text_element = TextElement::new(content, style, self.measure_text.clone());
        text_element.inherit_color(
            self.flex
                .inherited_color
                .unwrap_or(Color::default_foreground_color()),
        );
        self.flex.add_child(KaolinNode::new(text_element, None));
        self
    }
}
