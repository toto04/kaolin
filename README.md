# Kaolin

A flexible, immediate-mode UI layout library for Rust, inspired by [Clay](https://github.com/nicbarker/clay). Kaolin provides a powerful and intuitive API for creating responsive layouts that work across different rendering backends.

## Features

- **Immediate-mode UI**: Build layouts using a declarative, function-based API
- **Flexible layout engine**: Supports complex flex-box style layouts with growth, shrinking, and constraint-based sizing
- **Multi-backend support**: Works with Raylib, embedded graphics, and custom renderers
- **Type-safe styling**: Comprehensive styling system with compile-time safety
- **Text rendering**: Built-in text layout with wrapping and multi-line support
- **No-std compatible**: Can be used in embedded environments

## Quick Start

Add Kaolin to your `Cargo.toml`:

```toml
[dependencies]
kaolin = "0.1"

# For Raylib support
kaolin = { version = "0.1", features = ["raylib"] }

# For embedded graphics support  
kaolin = { version = "0.1", features = ["embedded"] }
```

## Basic Usage

Here's a simple "Hello, World!" example using the Raylib renderer:

```rust
use kaolin::{
    Kaolin, grow, sizing,
    renderers::KaolinRenderer,
    style::{
        FlexStyle, TextStyle,
        layout::{Alignment, Justification, Layout},
    },
};

fn main() {
    use kaolin::renderers::raylib::RaylibRenderer;
    use raylib::color::Color;

    let mut renderer = RaylibRenderer::new(800, 600);
    
    while !renderer.should_close() {
        renderer.draw(|k| {
            k.with(
                FlexStyle::new()
                    .background_color(Color::WHITE)
                    .layout(Layout::new()
                        .alignment(Alignment::Center)
                        .justification(Justification::Center))
                    .sizing(sizing!(grow!())),
                |k| {
                    k.text("Hello, World!", 
                           TextStyle::new()
                               .font_size(48.0)
                               .color(Color::BLACK))
                }
            )
        });
    }
}
```

## Core Concepts

### Layout Structure

Kaolin uses a tree-based layout system where each element can contain child elements. The layout is built using a functional, immediate-mode API:

```rust
kaolin.draw(|k| {
    k.with(container_style, |k| {
        k.text("Some text", text_style)
         .with(nested_container_style, |k| {
             k.text("Nested content", text_style)
         })
    })
})
```

### Sizing Modes

Kaolin supports several sizing modes for flexible layouts:

#### Fixed Sizing

```rust
// Fixed dimensions
sizing!(fixed!(200.0), fixed!(100.0))  // 200px width, 100px height
sizing!(fixed!(150.0))                 // 150px for both dimensions
```

#### Fit-to-Content Sizing

```rust
// Fits to content size
sizing!(fit!())                        // No constraints
sizing!(fit!(max_width))               // Maximum width constraint
sizing!(fit!(min_width, max_width))    // Min and max constraints
```

#### Growth-based Sizing

```rust
// Grows to fill available space
sizing!(grow!())                       // Default growth factor (1.0)
sizing!(grow!(2.0))                    // Custom growth factor
sizing!(grow!(1.0, min, max))          // Growth with constraints
```

### Layout Direction and Alignment

Control how child elements are arranged:

```rust
Layout::new()
    .direction(Direction::LeftToRight)     // or TopToBottom, RightToLeft, BottomToTop
    .alignment(Alignment::Center)          // Cross-axis: Start, End, Center, Stretch
    .justification(Justification::Center)  // Main-axis: Start, End, Center, SpaceBetween, SpaceAround
    .gap(10.0)                            // Spacing between children
```

### Styling

Style containers and text with comprehensive options:

```rust
// Container styling
FlexStyle::new()
    .background_color(Color::BLUE)
    .corner_radius(8.0)
    .padding(Padding::all(16.0))
    .border(Border::new().width(2.0).color(Color::BLACK))

// Text styling  
TextStyle::new()
    .font_size(24.0)
    .font_id(1)
    .color(Color::WHITE)
```

## Examples

### Responsive Layout

Create layouts that adapt to different screen sizes:

```rust
k.with(
    FlexStyle::new()
        .layout(Layout::new().direction(Direction::TopToBottom))
        .sizing(sizing!(grow!())),
    |k| {
        // Header
        k.with(
            FlexStyle::new()
                .background_color(Color::DARKBLUE)
                .sizing(sizing!(grow!(), fixed!(60.0))),
            |k| k.text("Header", TextStyle::new().color(Color::WHITE))
        )
        // Content area
        .with(
            FlexStyle::new()
                .background_color(Color::LIGHTGRAY)
                .sizing(sizing!(grow!())),
            |k| {
                k.text("Main Content", TextStyle::new())
            }
        )
        // Footer
        .with(
            FlexStyle::new()
                .background_color(Color::DARKGRAY)
                .sizing(sizing!(grow!(), fixed!(40.0))),
            |k| k.text("Footer", TextStyle::new().color(Color::WHITE))
        )
    }
)
```

### Multi-Column Layout

```rust
k.with(
    FlexStyle::new()
        .layout(Layout::new().gap(20.0))
        .sizing(sizing!(grow!())),
    |k| {
        // Sidebar
        k.with(
            FlexStyle::new()
                .background_color(Color::GRAY)
                .sizing(sizing!(fixed!(200.0), grow!())),
            |k| k.text("Sidebar", TextStyle::new())
        )
        // Main content
        .with(
            FlexStyle::new()
                .background_color(Color::WHITE)
                .sizing(sizing!(grow!())),
            |k| k.text("Main Content", TextStyle::new())
        )
    }
)
```

## Rendering Backends

### Raylib

Perfect for desktop applications and games:

```rust
use kaolin::renderers::raylib::RaylibRenderer;

let mut renderer = RaylibRenderer::new(800, 600);
while !renderer.should_close() {
    renderer.draw(|k| {
        // Your layout here
    });
}
```

### Embedded Graphics

Ideal for embedded systems and small displays:

```rust
use kaolin::renderers::embedded::EmbeddedRenderer;
use embedded_graphics_simulator::{SimulatorDisplay, Window};

let mut display = SimulatorDisplay::new(Size::new(128, 64));
let renderer = EmbeddedRenderer::new(&fonts, display.bounding_box());

renderer.onto(&mut display).draw(|k| {
    // Your layout here
});
```

### Custom Renderers

Implement the `KaolinRenderer` trait to create your own backend:

```rust
impl KaolinRenderer for MyRenderer {
    type Color = MyColor;
    
    fn draw(&mut self, draw_fn: impl Fn(Kaolin<Self::Color>)) {
        // Implementation
    }
}
```

## Advanced Features

### Text Wrapping

Kaolin automatically handles text wrapping within constrained widths:

```rust
k.with(
    FlexStyle::new().sizing(sizing!(fixed!(200.0), fit!())),
    |k| {
        k.text("This is a long text that will automatically wrap to multiple lines", 
               TextStyle::new())
    }
)
```

### Proportional Layouts

Use growth factors to create proportional layouts:

```rust
k.with(FlexStyle::new().sizing(sizing!(grow!(1.0))), |k| k)  // Takes 1/4 of space
 .with(FlexStyle::new().sizing(sizing!(grow!(3.0))), |k| k)  // Takes 3/4 of space
```

### Nested Layouts

Create complex layouts by nesting containers:

```rust
k.with(outer_container, |k| {
    k.with(inner_container_1, |k| {
        k.text("Content 1", style)
    })
    .with(inner_container_2, |k| {
        k.with(deeply_nested, |k| {
            k.text("Nested content", style)
        })
    })
})
```

## Performance

Kaolin is designed for immediate-mode rendering with minimal allocations:

- Layout calculations are performed each frame but are highly optimized
- Text measurement is cached when possible
- Memory allocations are minimized through careful API design
- No-std compatibility for resource-constrained environments

## Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.

## License

See the [LICENSE](LICENSE) file for details.

## Inspiration

Kaolin is heavily inspired by [Clay](https://github.com/nicbarker/clay), bringing similar concepts to the Rust ecosystem with type safety and zero-cost abstractions.
