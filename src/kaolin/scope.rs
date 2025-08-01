use crate::{
    elements::{KaolinElement, flexbox::FlexBox, text::TextElement},
    kaolin::MeasureTextFn,
    style::{FlexStyle, TextConfig},
};

// pub type DrawingFn = fn(KaolinScope<'_>);

pub struct KaolinScope<'frame, Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    flex: FlexBox<'frame, Color>,
    pub(crate) measure_text: &'frame MeasureTextFn<'frame, Color>,
}

impl<'frame, Color> KaolinScope<'frame, Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    pub fn new(
        flex: FlexBox<'frame, Color>,
        measure_text: &'frame MeasureTextFn<'frame, Color>,
    ) -> Self {
        KaolinScope { flex, measure_text }
    }

    pub fn with(
        mut self,
        style: FlexStyle<Color>,
        contents: impl Fn(KaolinScope<'frame, Color>) -> KaolinScope<'frame, Color>,
    ) -> Self {
        let child_flex = FlexBox::new(style);
        let child_scope = KaolinScope::new(child_flex, self.measure_text);
        let modified_scope = contents(child_scope);
        let child_flex = modified_scope.conclude();
        self.flex.add_child(KaolinElement::Flex(child_flex));
        self
    }

    pub(super) fn conclude(self) -> FlexBox<'frame, Color> {
        self.flex
    }

    pub fn text(mut self, content: &str, style: TextConfig<Color>) -> Self {
        let text_element = TextElement::new(content, style, self.measure_text);
        self.flex.add_child(KaolinElement::Text(text_element));
        self
    }
}

// layout calculations order:
// 1. fit the width --> when creating the node (children have already been added)
// 2. grow & shrink the width --> before the children, top down when all the tree is built
// 3. wrap the text --> as we grow, when dealing with text
// 4. fit the height --> when all children have grown, fit the height before returning
// 5. grow & shrink the height --> call once again after all the growing
// 6. position and align all elements

// -- Call the drawing function
// | -- Create children
// | -- Fit to width
// -- Grow width
// | -- Grow children width
// | -- Fit to height
// -- Grow height
// -- Position and align all elements
