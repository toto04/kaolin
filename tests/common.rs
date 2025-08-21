use kaolin::{
    commands::{RenderCommand, RenderCommands},
    style::{KaolinColor, TextStyle},
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TestColor {
    Black,
    #[default]
    Transparent,
    #[allow(dead_code)]
    Red,
}

impl KaolinColor<TestColor> for TestColor {
    fn default_foreground_color() -> Self {
        TestColor::Black
    }

    fn default_background_color() -> Self {
        TestColor::Transparent
    }
}

#[allow(dead_code)]
pub fn measure_text(text: &str, _config: &TextStyle<TestColor>) -> (f64, f64) {
    (text.len() as f64 * 10.0, 20.0)
}

#[allow(dead_code)]
pub fn assert_render_commands(
    output: RenderCommands<TestColor>,
    expected: Vec<RenderCommand<TestColor>>,
) {
    for (command, expected_command) in output.zip(expected.into_iter()) {
        assert_eq!(command, expected_command, "Render command mismatch");
    }
}
