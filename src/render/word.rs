
use crate::get_device_pixel_ratio;

use makepad_ttf_parser::*;
use makepad_font::*;
use makepad_trapezoidator::*;
use makepad_geometry::*;
use makepad_internal_iter::*;
use makepad_path::*;

use std::collections::HashMap;


#[derive(Clone)]
pub struct Word {
    pub font : Font,
    pub char_points: HashMap<usize, Vec<f32>>,
    //pub atlas_pages: Vec<FontAtlasPage>,
}

pub const ATLAS_SUBPIXEL_SLOTS: usize = 64;

#[derive(Clone)]
pub struct FontAtlasPage {
    pub dpi_factor: f32,
    pub font_size: f32,
    pub atlas_glyphs: Vec<[Option<FontAtlasGlyph>; ATLAS_SUBPIXEL_SLOTS]>
}

#[derive(Clone, Copy)]
pub struct FontAtlasGlyph {
    pub tx1: f32,
    pub ty1: f32,
    pub tx2: f32,
    pub ty2: f32,
}

impl Word {

    pub fn default() -> Word {
        static FONT: &'static [u8] = include_bytes!("../../assets/LiberationMono-Regular.ttf");
        let f : Result<Font> = parse_ttf(FONT);
        let char_points = HashMap::default();
        Word { font : f.unwrap(), char_points }
    }

    pub fn create_char_points_for_text(&mut self, text: &str) {
        let mut chars = vec![];
        let mut char_iter = text.chars();
        while let Some(c) = char_iter.next() {
            chars.push(c as usize);
            if !self.char_points.contains_key(&(c as usize)) {
                let points = self.get_char_points(c as usize);
                self.char_points.insert(c as usize, points);    
            }
        }
    }

    pub fn get_char_points_for_char(&self, unicode: &usize) -> Option<&Vec<f32>> {
        self.char_points.get(unicode)
    }

    pub fn get_advance_for_char(&self, unicode: usize) -> f32{
        let glyph_id = self.font.char_code_to_glyph_index_map[unicode];
        let glyph = &self.font.glyphs[glyph_id];
        glyph.horizontal_metrics.advance_width * 0.008
    }

    //Get bounds for font to adjust placment in height for fx gjpq chars

    pub fn get_char_points(&self, unicode: usize) -> Vec<f32> {
        let mut points : Vec<f32> = vec![];
        let glyph_id = self.font.char_code_to_glyph_index_map[unicode];

        let glyph = &self.font.glyphs[glyph_id];
        let mut trapezoidator = Trapezoidator::new();
/*
        let glyphtc = atlas_page.atlas_glyphs[todo.glyph_id][todo.subpixel_id].unwrap();
        let tx = glyphtc.tx1 * cx.fonts_atlas.texture_size.x + todo.subpixel_x_fract * atlas_page.dpi_factor;
        let ty = 1.0 + glyphtc.ty1 * cx.fonts_atlas.texture_size.y - todo.subpixel_y_fract * atlas_page.dpi_factor;
*/

            
        // this one needs pixel snapping
     //   let min_pos_x = walk_x + font_size_logical * glyph.bounds.p_min.x;
     //   let min_pos_y = pos.y - font_size_logical * glyph.bounds.p_min.y + self.text_style.font_size * self.text_style.top_drop;


        let font_size = 6.0;
        let font_scale_logical = font_size * 96.0 / (72.0 * self.font.units_per_em);
        let font_scale_pixels = font_scale_logical * get_device_pixel_ratio() as f32;

        let trapezoids = {
            //let font_scale_pixels = 0.009;
            let mut trapezoids = Vec::new();
            let trapezoidate = trapezoidator.trapezoidate(
                glyph
                    .outline
                    .commands()
                    .map({
                    move | command | {
                        command.transform(
                            &AffineTransformation::identity()
                                .translate(Vector::new(-glyph.bounds.p_min.x, -(glyph.bounds.p_min.y)/20.0))
                                .uniform_scale(font_scale_pixels)
                                .translate(Vector::new(0.0, 0.0)) //.translate(Vector::new(tx, ty))
                        )
                    }
                }).linearize(0.5),
            );
            trapezoids.extend_from_internal_iter(
                    trapezoidate
            );
            trapezoids
        };

//   X0
//Y0 |\
//   | \
//   |  \ X1
//   |   \
//   |    | Y1
//   |    | Y3
//   |   /
//   |  /
//   | /
//Y2 |/
    
        for trapezoid in trapezoids {
    
            points.push(trapezoid.xs[0]); 
            points.push(trapezoid.ys[0]);
            points.push(trapezoid.xs[1]); 
            points.push(trapezoid.ys[1]);
            points.push(trapezoid.xs[0]); 
            points.push(trapezoid.ys[2]);
    
            points.push(trapezoid.xs[0]); 
            points.push(trapezoid.ys[2]);
            points.push(trapezoid.xs[1]); 
            points.push(trapezoid.ys[1]);
            points.push(trapezoid.xs[1]); 
            points.push(trapezoid.ys[3]);
        }    
        points
    }
/*
    pub fn get_atlas_page_id(&mut self, dpi_factor: f32, font_size: f32) -> usize {
        for (index, sg) in self.atlas_pages.iter().enumerate() {
            if sg.dpi_factor == dpi_factor
                && sg.font_size == font_size {
                return index
            }
        }
        self.atlas_pages.push(FontAtlasPage {
            dpi_factor: dpi_factor,
            font_size: font_size,
            atlas_glyphs: {
                let mut v = Vec::new();
                v.resize(self.font.glyphs.len(), [None; ATLAS_SUBPIXEL_SLOTS]);
                v
            }
        });
        self.atlas_pages.len() - 1
    }
    */
}
