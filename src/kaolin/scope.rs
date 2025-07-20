use crate::{
    elements::{
        KaolinElement,
        flexbox::FlexBox,
        text::{TextConfig, TextElement},
    },
    style::FlexStyle,
};

// pub type DrawingFn = fn(KaolinScope<'_>);

pub struct KaolinScope<'a> {
    flex: FlexBox<'a>,
}

impl<'a> KaolinScope<'a> {
    pub fn new(flex: FlexBox<'a>) -> Self {
        KaolinScope { flex }
    }

    pub fn with(&mut self, style: FlexStyle, contents: fn(KaolinScope<'_>) -> KaolinScope<'_>) {
        let child_flex = FlexBox::new(style);
        let child_scope = KaolinScope::new(child_flex);
        let child_flex = contents(child_scope).extract();
        self.flex.add_child(KaolinElement::Flex(child_flex));
    }

    pub(super) fn extract(self) -> FlexBox<'a> {
        self.flex
    }

    pub fn text(&mut self, content: &str, style: TextConfig) {
        let text_element = TextElement::new(content, style);
        self.flex.add_child(KaolinElement::Text(text_element));
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
