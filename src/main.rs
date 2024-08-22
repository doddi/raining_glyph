use std::fs::read_to_string;
use std::time::Duration;

use anathema::backend::Backend;
use anathema::component::*;
use anathema::prelude::*;
use anathema::state::Hex;
use colorsys::{ColorTransform, Hsl, Rgb};
use rand::Rng;

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

struct Canvas;

impl Canvas {
    pub fn new() -> Self {
        Canvas {}
    }
}

impl Component for Canvas {
    type State = CanvasState;
    type Message = ();
}

static CHARS: &str = "ﾊﾐﾋｰｳｼﾅﾓﾆｻﾜﾂｵﾘｱﾎﾃﾏｹﾒｴｶｷﾑﾕﾗｾﾈｽﾀﾇﾍｦｲｸｺｿﾁﾄﾉﾌﾔﾖﾙﾚﾛﾝ012345789Z:.\"=*+-<>¦╌ç";
#[derive(State)]
struct Glyph {
    character: Value<char>,
    colour: Value<Hex>,
}

impl Glyph {
    fn new() -> Self {
        Glyph {
            character: Value::new(' '),
            colour: Value::new((0, 255, 0).into()),
        }
    }

    fn new_random() -> Self {
        let i = rand::thread_rng().gen_range(0..CHARS.chars().count());
        let character = CHARS.chars().nth(i).unwrap();
        let colour = (0, 255, 0).into();

        Glyph {
            character: Value::new(character),
            colour: Value::new(colour),
        }
    }

    fn fade_colour(&mut self) {
        let colour = self.colour.copy_value();
        let mut hsl = Hsl::from(Rgb::from((colour.r, colour.g, colour.b)));
        hsl.lighten(-1.0);
        let rgb = Rgb::from(hsl);
        let hex = (rgb.red() as u8, rgb.green() as u8, rgb.blue() as u8).into();

        self.colour.set(hex);
    }

    fn randomly_change_character(&mut self) {
        let colour = self.colour.copy_value();
        let hsl = Hsl::from(Rgb::from((colour.r, colour.g, colour.b)));

        // Only change the characters while it is slightly visible
        if hsl.saturation() < 10.0 || hsl.lightness() < 10.0 {
            return;
        }
        let i = rand::thread_rng().gen_range(0..CHARS.chars().count());
        self.character.set(CHARS.chars().nth(i).unwrap());
    }
}

impl Component for Glyph {
    type State = ();
    type Message = ();
}

#[derive(State)]
struct GlyphColumnState {
    glyphs: Value<List<Glyph>>,
    trail_start: Value<usize>,
}

impl GlyphColumnState {
    pub fn new(height: usize) -> Self {

        let mut glyphs = List::empty();
        for _ in 0..height {
            glyphs.push(Value::new(Glyph::new()));
        }

        GlyphColumnState {
            glyphs: glyphs,
            trail_start: Value::new(0),
        }
    }
}

struct GlyphColumn;

impl GlyphColumn {
    pub fn new() -> Self {
        GlyphColumn {}
    }
}

impl Component for GlyphColumn {
    type State = GlyphColumnState;
    type Message = ();

    fn tick(&mut self, state: &mut Self::State, _elements: Elements<'_, '_>, _context: Context<'_>, _dt: Duration) {
        // state.glyphs.for_each(|glyph| {
        //     glyph.fade_colour();
        //     glyph.randomly_change_character();
        // });

        // Only update the first glyph in the column at a random interval time
        if state.trail_start.copy_value() == 0 && rand::thread_rng().gen_range(0..100) > 1 {
            return;
        }

        state.trail_start.set(state.trail_start.copy_value() + 1);
        if state.trail_start.copy_value() == state.glyphs.len() {
            state.trail_start.set(0);
        }

        state.glyphs.remove(state.glyphs.len()-1);
        state.glyphs.insert(0, Value::new(Glyph::new_random()));
    }
}


fn main() {
    let template = read_to_string("src/templates/rain.aml").unwrap();

    let doc = Document::new(template);

    let backend = TuiBackend::builder()
        // .enable_alt_screen()
        // .enable_raw_mode()
        // .hide_cursor()
        .finish()
        .unwrap();

    let size = backend.size();
    let mut runtime = Runtime::builder(doc, backend);

    runtime.register_prototype(
        "glyph",
        "src/templates/glyph.aml",
        || Glyph::new(),
        || (),
    ).unwrap();

    runtime.register_prototype(
        "glyph_column",
        "src/templates/glyph_column.aml",
        || GlyphColumn::new(),
        move || GlyphColumnState::new(size.height)
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
