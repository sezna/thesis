extern crate palette;
extern crate rsgenetic;
extern crate image;
extern crate rand;
mod stroke;
use stroke::Painting;
use rsgenetic::sim::seq::Simulator;
use rsgenetic::sim::*;
use rsgenetic::sim::select::*;

fn main() {
    let mut population = (0..40).map(|_| Painting::informed_random("redgreenfuzzy.bmp", 300)).collect();
    let mut s = Simulator::builder(&mut population)
                    .set_selector(Box::new(MaximizeSelector::new(10)))
                    .set_max_iters(25)
                    .build();
    s.run();
    println!("the most fit member is: {}", s.get().unwrap().fitness());
    s.get().unwrap().render_and_save_image();
    

}
