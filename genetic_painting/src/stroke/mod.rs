pub mod point_2d;
use self::point_2d::Point2D;
// use palette::Rgb;
use rsgenetic::pheno::*;
use std::path::Path;
use image;
use rand::Rng;
use rand::thread_rng;
use imageproc::drawing::*;

#[derive(Clone)]
pub struct Stroke {
    start: Point2D,
    end: Point2D,
    color: image::Rgb<u8>,
    width: u32,
}

#[derive(Clone)]
pub struct Painting {
    strokes: Vec<Stroke>,
    width: u32,
    height: u32,
    filename: String,
}

impl Painting {
    // Generates a Painting where the strokes are always the color of the pixel
    // that they start or
    // end in. Size is the number of strokes.
    pub fn informed_random(filename: &str, size: i32) -> Painting {
        let image = load_image(filename);
        let num_of_pixels = image.height() * image.width();
        let pixels_per_stroke = num_of_pixels / size as u32;
        let mut rng = thread_rng();
        let mut count = 0;
        let mut strokes: Vec<Stroke> = Vec::new();
        for _ in 0..num_of_pixels {
            count += 1;
            if count == pixels_per_stroke {
		let mut stroke_length = (image.height() + image.width()) as f64;
		let mut start = Point2D::default();
 		let mut end = Point2D::default();
		while stroke_length > image.height() as f64 / 2.0 && stroke_length > image.width() as f64 / 2.0 {
			start = Point2D {
			    x: (rng.gen::<u32>() % image.width()),
			    y: (rng.gen::<u32>() % image.height()),
			};
			end = Point2D {
			    x: (rng.gen::<u32>() % image.width()),
			    y: (rng.gen::<u32>() % image.height()),
			};
			stroke_length = f64::sqrt(((end.x - start.x) * (end.x - start.x) + (end.y - start.y) * (end.y - start.y)) as f64); 
		}

                let rgb = image.get_pixel(start.x, start.y);
                count = 0;
                strokes.push(Stroke {
                    start: start,
                    end: end,
                    color: rgb.clone(),
                    width: rng.gen::<u32>() % 5 + 1, /* TODO how do I determine what I want width to
                                                  * be? */
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

    fn render_strokes(&self) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        // render strokes.
        let mut rendered_strokes_buffer = image::ImageBuffer::<image::Rgb<u8>,
                                                               Vec<u8>>::new(self.width,
                                                                             self.height);
        for stroke in self.strokes.iter() {
//          draw_line_segment::<image::ImageBuffer<image::Rgb<u8>,
//           Vec<u8>>>(rendered_strokes_buffer, stroke.start, stroke.end, stroke.color);


            draw_line_segment_mut(&mut rendered_strokes_buffer, (stroke.start.x as f32, stroke.start.y as f32), (stroke.end.x as f32, stroke.end.y as f32), stroke.color);
            /*
            let slope: f64 = (stroke.end.y as f64 - stroke.start.y as f64) /
                             (stroke.end.x as f64 - stroke.start.x as f64);

            let mut index = 0i64;
        //    println!("stroke width: {}", stroke.width);
            for i in 0..stroke.width {
                for x in stroke.start.x + i..stroke.end.x + i {
                    let y = stroke.start.y + i + (index as f64 * slope) as u32;
                    if  x < self.width && y < self.height {
                        rendered_strokes_buffer.put_pixel(x, y, stroke.color);
                    }
                    index += 1;
                }
            } 
            */
        }
        return rendered_strokes_buffer;
    }

    pub fn render_and_save_image(&self) {
        println!("saving image...");
        let _ = self.render_strokes().save(&Path::new("fittest.png"));
    }
	
    pub fn render_painting(&self, path: &str) {
        println!("saving image...");
        let _ = self.render_strokes().save(&Path::new(path));
    }


    // TODO put filename in the painting somehow
    pub fn fitness(&self) -> i32 {
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

impl Phenotype<i32> for Painting  {
    fn fitness(&self) -> i32 {
        return self.fitness();
    }

    // The "mating" function
    fn crossover(&self, other: &Painting) -> Painting {
        let s = self.clone();
        let o = other.clone();
        let (half_of_self, _) = s.strokes.split_at(self.strokes.len() / 2);
        let (_, half_of_other) = o.strokes.split_at(self.strokes.len() / 2);

        return Painting {
            strokes: [half_of_self, half_of_other].concat(),
            width: self.width,
            height: self.height,
            filename: s.filename,
        };
        // TODO: intelligent crossover, pick the most fit strokes.
    }

    // randomly change some strokes. perhaps mutation should be dramatic.
    fn mutate(&self) -> Painting {
        let mut rng = thread_rng();
        let mut s = self.clone();
        let start = Point2D {
            x: (rng.gen::<u32>() % self.width),
            y: (rng.gen::<u32>() % self.height),
        };
        let end = Point2D {
            x: (rng.gen::<u32>() % self.width),
            y: (rng.gen::<u32>() % self.height),
        };

        let image = load_image(&self.filename);
        let rgb = image.get_pixel(start.x, start.y);
        println!("mutation added: {} {} {}", rgb.data[0], rgb.data[1], rgb.data[2]);
        s.strokes.push(Stroke {
            start: start,
            end: end,
            color: rgb.clone(),
            width: rng.gen::<u32>() % 5 + 1,
        });
        return s;
    }
}

fn load_image(filename: &str) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    return image::open(&Path::new(filename))
               .expect("invalid filename when loading image")
               .to_rgb();
}
