use kaolin::commands::RenderCommand;
use kaolin::renderers::KaolinRenderer;
use kaolin::renderers::raylib::RaylibRenderer;
use kaolin::style::colors::Colors;

fn main() {
    RaylibRenderer::draw_fn(|renderer: &mut RaylibRenderer| {
        let commands = vec![
            RenderCommand::DrawRectangle {
                x: 10,
                y: 20,
                width: 100,
                height: 50,
                color: Colors::Red.into(),
            },
            RenderCommand::DrawText {
                text: "Hello, World!",
                x: 15,
                y: 25,
                font_size: 20,
                color: Colors::White.into(),
            },
        ];
        renderer.draw(commands);
    });
}
