use kaolin::style::{KaolinColor, TextStyle};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TestColor {
    Black,
    #[default]
    Transparent,
    #[allow(dead_code)]
    Red,
}

impl KaolinColor for TestColor {
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

#[macro_export]
macro_rules! assert_render_commands {
    ($output:expr, $expected:expr $(,)?) => {{
        let o: kaolin::commands::RenderCommands<TestColor, ()> = $output;
        let e: Vec<kaolin::commands::RenderCommand<TestColor, ()>> = $expected;
        for (command, expected_command) in o.zip(e.into_iter()) {
            assert_eq!(command, expected_command, "Render command mismatch");
        }
    }};
}

#[macro_export]
macro_rules! assert_position {
    ($command:expr, ($x:expr, $y:expr)) => {{
        let c: kaolin::commands::RenderCommand<TestColor, ()> = $command.unwrap();
        let (x, y) = match c {
            kaolin::commands::RenderCommand::DrawRectangle { x, y, .. } => (x, y),
            kaolin::commands::RenderCommand::DrawText { x, y, .. } => (x, y),
            _ => panic!("Expected DrawRectangle or DrawText command"),
        };
        assert_eq!(x, $x, "X position mismatch");
        assert_eq!(y, $y, "Y position mismatch");
    }};
}

#[macro_export]
macro_rules! assert_size {
    ($command:expr, ($w:expr, _)) => {{
        let c: kaolin::commands::RenderCommand<TestColor, ()> = $command.unwrap();
        match c {
            kaolin::commands::RenderCommand::DrawRectangle { width, .. } => {
                assert_eq!(width, $w, "Width mismatch");
            }
            _ => panic!("Expected DrawRectangle command"),
        }
    }};
    ($command:expr, (_, $h:expr)) => {{
        let c: kaolin::commands::RenderCommand<TestColor, ()> = $command.unwrap();
        match c {
            kaolin::commands::RenderCommand::DrawRectangle { height, .. } => {
                assert_eq!(height, $h, "Height mismatch");
            }
            _ => panic!("Expected DrawRectangle command"),
        }
    }};

    ($command:expr, ($w:expr, $h:expr)) => {{
        let c: kaolin::commands::RenderCommand<TestColor, ()> = $command.unwrap();
        match c {
            kaolin::commands::RenderCommand::DrawRectangle { width, height, .. } => {
                assert_eq!(width, $w, "Width mismatch");
                assert_eq!(height, $h, "Height mismatch");
            }
            _ => panic!("Expected DrawRectangle command"),
        }
    }};
}

#[macro_export]
macro_rules! assert_text_content {
    ($command:expr, $text:expr) => {{
        let c: kaolin::commands::RenderCommand<TestColor, ()> = $command.unwrap();
        match c {
            kaolin::commands::RenderCommand::DrawText { text, .. } => {
                assert_eq!(text, $text, "Text content mismatch");
            }
            _ => panic!("Expected DrawText command"),
        }
    }};
}

#[macro_export]
macro_rules! assert_color {
    ($command:expr, $color:expr) => {{
        let c: kaolin::commands::RenderCommand<TestColor, ()> = $command.unwrap();
        let color = match c {
            kaolin::commands::RenderCommand::DrawRectangle { color, .. } => color,
            kaolin::commands::RenderCommand::DrawText { color, .. } => color,
            _ => panic!("Expected DrawRectangle or DrawText command"),
        };
        assert_eq!(color, $color, "Color mismatch");
    }};
}

#[macro_export]
macro_rules! assert_multiple {
    ($command:expr, $($statement:ident($arg:tt)),+ $(,)?) => {{
        let command: kaolin::commands::RenderCommand<TestColor, ()> = $command.unwrap();
        $(
            $statement!(Some(command.clone()), $arg);
        )+
    }};
}
