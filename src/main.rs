use std::fs::read_to_string;
use std::time::Duration;
use anathema::backend::Backend;
use anathema::component::*;
use anathema::prelude::*;
use anathema::state::Hex;

#[derive(State)]
struct CanvasState {
    columns: Value<List<usize>>,
}

impl CanvasState {
    pub fn new(width: impl IntoIterator<Item = usize>) -> Self {
        CanvasState {
            columns: List::from_iter(width),
        }
    }
}

struct Canvas {}

impl Canvas {
    pub fn new() -> Self {
        Canvas {}
    }
}

impl Component for Canvas {
    type State = CanvasState;
    type Message = ();
}

#[derive(State)]
struct GlyphState {
    character: Value<char>,
    colour: Value<Hex>,
}

impl GlyphState {
    pub fn new() -> Self {
        GlyphState {
            character: Value::new('O'),
            colour: Value::new((0, 255, 0).into()),
        }
    }
}

struct Glyph {}

impl Glyph {
    pub fn new() -> Self {
        Glyph {}
    }
}

impl Component for Glyph {
    type State = GlyphState;
    type Message = ();

    fn tick(&mut self, _state: &mut Self::State, _elements: Elements<'_, '_>, _context: Context<'_>, _dt: Duration) {
    }
}

#[derive(State)]
struct GlyphColumnState {
    glyphs: Value<List<usize>>,
}

impl GlyphColumnState {
    pub fn new(height: impl IntoIterator<Item = usize>) -> Self {
        GlyphColumnState {
            glyphs: List::from_iter(height),
        }
    }
}

struct GlyphColumn {
}

impl GlyphColumn {
    pub fn new() -> Self {
        GlyphColumn {}
    }
}

impl Component for GlyphColumn {
    type State = GlyphColumnState;
    type Message = ();

    fn tick(&mut self, _state: &mut Self::State, _elements: Elements<'_, '_>, _context: Context<'_>, _dt: Duration) {
    }
}

fn main() {
    let template = read_to_string("src/templates/rain.aml").unwrap();

    let doc = Document::new(template);

    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();

    let size = backend.size();
    let mut runtime = Runtime::builder(doc, backend);

    runtime.register_prototype(
        "glyph",
        "src/templates/glyph.aml",
        || Glyph::new(),
        || GlyphState::new(),
    ).unwrap();

    runtime.register_prototype(
        "glyph_column",
        "src/templates/glyph_column.aml",
        || GlyphColumn::new(),
        move || GlyphColumnState::new(0..size.height)
    ).unwrap();

    runtime.register_prototype(
        "canvas",
        "src/templates/canvas.aml",
        || Canvas::new(),
        move || CanvasState::new(0..size.width)
    ).unwrap();
    let mut runtime = runtime.finish().unwrap();
    runtime.run();
}