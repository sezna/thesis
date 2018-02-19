pub mod point_2d;
use self::point_2d::Point2D;
// use palette::Rgb;
use rsgenetic::pheno::*;
use std::path::Path;
use std::thread;
use image;
use rand::Rng;
use rand::thread_rng;
use imageproc::drawing::*;

/// Represents one stroke in a painting.
#[derive(Clone)]
pub struct Stroke {
    start: Point2D,
    end: Point2D,
    controls: (Point2D, Point2D),
    color: image::Rgb<u8>,
    width: u32,
}

/// Represents a collection of strokes forming a painting.
#[derive(Clone)]
pub struct Painting {
    strokes: Vec<Stroke>,
    width: u32,
    height: u32,
    filename: String,
}

impl Painting {
    /// Generates a Painting where the strokes are always the color of the pixel
    /// that they start or end in. Size is the number of strokes. Min/Max length
    /// are the minimum and maximum lengths any stroke can be.
    pub fn informed_random(filename: &str, number_of_strokes: u32, width: u32, minlength: u32, maxlength: u32) -> Painting {
        println!("generating");
        let image = load_image(filename);
        let num_of_pixels = image.height() * image.width();
        let pixels_per_stroke = num_of_pixels / number_of_strokes;
        let mut rng = thread_rng();
        let mut count = 0;
        let mut strokes: Vec<Stroke> = Vec::new();
        for _ in 0..num_of_pixels {
            count += 1;
            if count == pixels_per_stroke {
                let mut stroke_length = (image.height() + image.width()) as f64;
                let mut start = Point2D::default();
                let mut end = Point2D::default();
                let mut control_a = Point2D::default();
                let mut control_b = Point2D::default();
                while stroke_length <= minlength as f64 || stroke_length >= maxlength as f64{
                    start = Point2D {
                        x: (rng.gen::<u32>() % image.width()),
                        y: (rng.gen::<u32>() % image.height()),
                    };
                    end = Point2D {
                        x: (rng.gen::<u32>() % image.width()),
                        y: (rng.gen::<u32>() % image.height()),
                    };
                    control_a = Point2D {
                        x: (rng.gen::<u32>() % image.width()),
                        y: (rng.gen::<u32>() % image.height()),
                    };
                    control_b = Point2D {
                        x: (rng.gen::<u32>() % image.width()),
                        y: (rng.gen::<u32>() % image.height()),
                    };
                    stroke_length =
                        f64::sqrt(((end.x as f64 - start.x as f64) *
                                   (end.x as f64 - start.x as f64) +
                                   (end.y as f64 - start.y as f64) *
                                   (end.y as f64 -
                                    start.y as f64)) as f64);
                } // TODO really fix those "as f64" things

                let rgb = image.get_pixel(start.x, start.y);
		
                count = 0;
		
                strokes.push(Stroke {
                    start: start,
                    end: end,
		    controls: (control_a, control_b),
                    color: rgb.clone(),
                    width: rng.gen::<u32>() % width + 1, /* TODO how do I determine what I want
                                                          * width to be? */
                });
            }
        }

        return Painting {
            strokes: strokes,
            width: image.width(),
            height: image.height(),
            filename: filename.to_string(),
        };

    }

    /// Randomly generates a lot of strokes within the boundaries of of the size of the input image.
    /// Width is the width of each stroke, min/max length control how short or long each line can be.
    pub fn random(filename: &str, number_of_strokes: u32, width: u32, minlength: u32, maxlength: u32, maxcurve: u32) -> Painting {
        let image = load_image(filename);
        let num_of_pixels = image.height() * image.width();
        let pixels_per_stroke = num_of_pixels / number_of_strokes;
        let mut rng = thread_rng();
        let mut count = 0;
        let mut strokes: Vec<Stroke> = Vec::new();

        // To achieve an evenly distributed spread of strokes, we iterate through all pixels and 
        // generate one every `pixels_per_stroke` pixels.
        for _ in 0..num_of_pixels {
            count += 1;
            if count == pixels_per_stroke {
                let mut stroke_length = (image.height() + image.width()) as f64;
                let mut start = Point2D::default();
                let mut end = Point2D::default();
                // Control points are for the cubic bezier draw.
        		let mut control_a = Point2D::default();
        		let mut control_b = Point2D::default();

                // Hacky, but continue trying until a stroke has been picked that is within the length bounds. This is in parallel anyway.
                while stroke_length < minlength as f64 ||
                      stroke_length > maxlength as f64 {
                    start = Point2D {
                        x: (rng.gen::<u32>() % image.width()),
                        y: (rng.gen::<u32>() % image.height()),
                    };
                    end = Point2D {
                        x: (rng.gen::<u32>() % image.width()),
                        y: (rng.gen::<u32>() % image.height()),
                    };

                    let slope = Point2D { x: if end.y < start.y { start.y - end.y } else { end.y - start.y }, y: if end.x < start.x { start.x - end.x } else { end.x - start.x } };
                    control_a = Point2D {
                        x: start.x + slope.x + maxcurve,
                        y: start.y + slope.y + maxcurve};
                    control_b = Point2D {
                        x: start.x + slope.x + maxcurve,
                        y: start.y + slope.y + maxcurve};
                    };

                    stroke_length =
                        f64::sqrt(((end.x - start.x) * (end.x - start.x) +
                                   (end.y - start.y) *
                                   (end.y - start.y)) as f64);
                }

                let rgb = image.get_pixel(rng.gen::<u32>() % image.width(),
                                          rng.gen::<u32>() % image.height()); // or should this be truly random?
                count = 0;

                // Finally, push the generated stroke onto the vector of strokes.
                strokes.push(Stroke {
                    start: start,
                    end: end,
		    controls: (control_a, control_b),
                    color: rgb.clone(),
                    width: rng.gen::<u32>() % width + 1, /* TODO how do I determine what I want
                                                          * width to be? */
                });
            }
        }


        return Painting {
            strokes: strokes,
            width: image.width(),
            height: image.height(),
            filename: filename.to_string(),
        };
    }

    /// Render the currect strokes into an Imagebuffer.
    fn render_strokes(&self) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let mut rendered_strokes_buffer = image::ImageBuffer::<image::Rgb<u8>,
                                                               Vec<u8>>::new(self.width,
                                                                             self.height);
        // draw the line with width taken into account.
        for stroke in self.strokes.iter() {
            for i in 0..stroke.width {
/*
                draw_line_segment_mut(&mut rendered_strokes_buffer,
                                      (stroke.start.x as f32 + i as f32,
                                       stroke.start.y as f32 + i as f32),
                                      (stroke.end.x as f32 + i as f32, stroke.end.y as f32 + i as f32),
                                      stroke.color);
*/
        // for testing right now, draw a curve after drawing the line.
		draw_cubic_bezier_curve_mut(&mut rendered_strokes_buffer,
					(stroke.start.x as f32 + i as f32,
					stroke.start.y as f32 + i as f32),
					(stroke.end.x as f32 + i as f32, stroke.end.y as f32 + i as f32),
					stroke.controls.0.as_tuple(), stroke.controls.1.as_tuple(), stroke.color);
            }
        }
        return rendered_strokes_buffer;
    }

    /// Save a painting to an image.
    pub fn render_and_save_image(&self, filename: String) {
        println!("saving image...");
        let _ = self.render_strokes().save(Path::new(&filename));
    }

    /// Save a painting to a custom filepath.
    pub fn render_painting(&self, path: &str) {
        println!("saving image...");
        let _ = self.render_strokes().save(&Path::new(path));
    }


    pub fn fitness(&self) -> i32 {
	println!("in stroke fitness");
        let mut fitness = 0f64;
        // The image we are trying to approximate.
        let goal = load_image(&self.filename);
        let rendered_strokes_buffer = self.render_strokes();
        for x in 0..goal.width() {
            for y in 0..goal.height() {
                let grgb = goal.get_pixel(x, y).data;//.iter().collect();//.map(|x| x as i32);
                let rrgb = rendered_strokes_buffer.get_pixel(x, y);
                let unfitness = (grgb[0] as i32 - rrgb[0] as i32).abs() +
                                (grgb[1] as i32 - rrgb[1] as i32).abs() +
                                (grgb[2] as i32 - rrgb[2] as i32).abs();
                fitness += 765.0 - unfitness as f64;

            }
        }

        // println!("evaluated fitness as {}", fitness);
        return fitness as i32;
    }

}

/// Used for the RsGenetic crate. 
impl Phenotype<i32> for Painting  {

    /// Calculates the fitness from an integer. Conveniently, fitness is an integer.
    fn fitness(&self) -> i32 {
	println!("in phenotype fitness");
        return self.fitness();
    }

    /// The "mating" function for the genetic algorithm.
    fn crossover(&self, other: &Painting) -> Painting {
        println!("mating");
        let s = self.clone();
        let o = other.clone();
        let (half_of_self, _) = s.strokes.split_at(self.strokes.len() / 2);
        let (_, half_of_other) = o.strokes.split_at(self.strokes.len() / 2);

        let p1 = Painting {
            strokes: [half_of_self, half_of_other].concat(),
            width: self.width,
            height: self.height,
            filename: s.filename.clone(),
        };

        let p2 = Painting {
            strokes: [half_of_other, half_of_self].concat(),
            width: self.width,
            height: self.height,
            filename: s.filename,
        };


        // TODO remove these clones

        let p1c = p1.clone();
        let p2c = p2.clone();

        let p1fitness = thread::spawn(move || { p1c.fitness() });
        let p2fitness = thread::spawn(move || { p2c.fitness() });
        if p1fitness.join().expect("thread failed") > p2fitness.join().expect("thread failed") { return p1; } else { return p2; }
        // TODO: intelligent crossover, pick the most fit strokes.
    }

    // randomly change some strokes. perhaps mutation should be dramatic.
    fn mutate(&self) -> Painting {
        println!("mutating");
        let mut rng = thread_rng();
        let mut s = self.clone();
        let pre = self.fitness();
	let to_modify_index = rng.gen::<usize>() % self.strokes.len();
	println!("strokes len: {}, to_modify_index: {}", self.strokes.len(), to_modify_index);
	let mut to_modify = self.strokes[to_modify_index].clone();

	// Decide which part of the stroke we will modify.
	match rng.gen::<i32>() % 3 {
		0 => {to_modify.start.x = (to_modify.start.x + rng.gen::<u32>() % 5) % self.width; to_modify.start.y = (to_modify.start.y + rng.gen::<u32>() % 5) % self.height;},
		1 => {to_modify.end.x = (to_modify.end.x + rng.gen::<u32>() % 5) % self.width; to_modify.end.y = (to_modify.end.y + rng.gen::<u32>() % 5) % self.height;},
		2 => {to_modify.width = to_modify.width + rng.gen::<u32>() % 5;},
		_ => (),

	}

	s.strokes.remove(to_modify_index);
	s.strokes.push(to_modify);
    let post = s.fitness();
    if post > pre { return s; } else { return self.clone(); }
    }
}

/// Load an image from the given file name. 
fn load_image(filename: &str) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    return image::open(&Path::new(filename))
               .expect("invalid filename when loading image")
               .to_rgb();
}
