
use makepad_ttf_parser::*;
use makepad_font::*;
use makepad_trapezoidator::*;
use makepad_geometry::*;
use makepad_internal_iter::*;
use makepad_path::*;

pub struct Fonts {

    pub font: Font,

}

impl Fonts {

    pub fn default() -> Fonts {
        static FONT: &'static [u8] = include_bytes!("../../assets/LiberationMono-Regular.ttf");
        let f : Result<Font> = parse_ttf(FONT);
        Fonts { font : f.unwrap() }
    }

    pub fn get_char_points(&self, unicode: usize) -> Vec<f32> {
        let mut points : Vec<f32> = vec![];
        let glyph_id = self.font.char_code_to_glyph_index_map[unicode];

        let glyph = &self.font.glyphs[glyph_id];
        let outline = &glyph.outline;
        let outline_points = outline.points();
    
        for outline_point in outline_points {
            if outline_point.is_on_curve {
                let point = outline_point.point;
                points.push(point.x);
                points.push(point.y);
            }    
        }
    
        let rect = &glyph.bounds;
    
        let mut trapezoidator = Trapezoidator::new();
    
    
        let mut points : Vec<f32> = vec![];
    
        let trapezoids = {
            let font_scale_pixels = 0.15;
            let mut trapezoids = Vec::new();
            let trapezoidate = trapezoidator.trapezoidate(
                glyph
                    .outline
                    .commands()
                    .map({
                    move | command | {
                        command.transform(
                            &AffineTransformation::identity()
                                .translate(Vector::new(-glyph.bounds.p_min.x, -glyph.bounds.p_min.y))
                                .uniform_scale(font_scale_pixels)
                                .translate(Vector::new(0.0, 0.0))
                        )
                    }
                }).linearize(0.5),
            );
            trapezoids.extend_from_internal_iter(
                    trapezoidate
            );
            trapezoids
        };
    //    Y     Y
    //    
    //    A     B
    // X  _______
    //    |\    |
    //    | \   |
    //    |  \  |
    //    |   \ |
    //    |    \|
    // X  -------
    //    D      C
    
        for trapezoid in trapezoids {
    
            points.push(trapezoid.xs[1]); //A
            points.push(trapezoid.ys[0]);
            points.push(trapezoid.xs[1]); //B
            points.push(trapezoid.ys[1]);
            points.push(trapezoid.xs[0]); //C
            points.push(trapezoid.ys[2]);
    
            points.push(trapezoid.xs[1]); //A
            points.push(trapezoid.ys[0]);
            points.push(trapezoid.xs[0]); //D
            points.push(trapezoid.ys[3]);
            points.push(trapezoid.xs[0]); //C
            points.push(trapezoid.ys[2]);
    
        }    
        points
    }
}