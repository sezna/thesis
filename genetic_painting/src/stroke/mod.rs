pub mod point_2d;
use self::point_2d::Point2D;
// use palette::Rgb;
use rsgenetic::pheno::*;
use std::path::Path;
use image;
use rand::Rng;
use rand::thread_rng;

#[derive(Clone)]
pub struct Stroke {
    start: Point2D,
    end: Point2D,
    color: image::Rgb<u8>,
}

#[derive(Clone)]
pub struct Painting {
    strokes: Vec<Stroke>,
}

impl Painting {
    // Generates a random Painting with width, height, and size number of strokes.
    pub fn random(width: u32, height: u32, size: i32) -> Painting {
        let mut strokes: Vec<Stroke> = Vec::new();
        let mut rng = thread_rng();
        for _ in 0..size {
            let start = Point2D {
                x: (rng.gen::<u32>() % width),
                y: (rng.gen::<u32>() % height),
            };
            let end = Point2D {
                x: (rng.gen::<u32>() % width),
                y: (rng.gen::<u32>() % height),
            };
            let color = image::Rgb::<u8> {
                // data: [(rng.gen::<u8>() % 255), (rng.gen::<u8>() % 255), (rng.gen::<u8>() %
                // 255)],
                data: [rng.gen::<u8>() % 255, rng.gen::<u8>() % 255, rng.gen::<u8>() % 255],
            };
            strokes.push(Stroke {
                start: start,
                end: end,
                color: color,
            });
        }

        return Painting { strokes: strokes };
    }


    fn render_strokes(&self) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {

        let goal = image::open(&Path::new("Red.png")).unwrap().to_rgb();
        let (width, height) = goal.dimensions();

        // render strokes.
        let mut rendered_strokes_buffer = image::ImageBuffer::<image::Rgb<u8>,
                                                               Vec<u8>>::new(width, height);
        for stroke in self.strokes.iter() {
            let slope: f64 = (stroke.end.y as f64 - stroke.start.y as f64) /
                             (stroke.end.x as f64 - stroke.start.x as f64);

            let mut index = 0i64;
            for x in stroke.start.x..stroke.end.x {
                let y = stroke.start.y + (index as f64 * slope) as u32;
                rendered_strokes_buffer.put_pixel(x, y, stroke.color);
                if index > 12000000 {
                    println!("index: {}", index);
                }
                index += 1;
            }
        }
        return rendered_strokes_buffer;
    }

    pub fn render_and_save_image(&self) {
        println!("saving image...");
        let _ = self.render_strokes().save(&Path::new("fittest.png"));
    }

    pub fn fitness(&self) -> i32 {
        let mut fitness = 0f64;
        // The image we are trying to approximate.
        let goal: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::open(&Path::new("Red.png"))
                                                                    .unwrap()
                                                                    .to_rgb();
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

        return Painting { strokes: [half_of_self, half_of_other].concat() };
        // TODO: intelligent crossover, pick the most fit strokes.
    }

    // randomly change some strokes. perhaps mutation should be dramatic.
    fn mutate(&self) -> Painting {
        return self.clone();
    }
}
