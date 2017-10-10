extern crate palette;
extern crate imageproc;
extern crate rsgenetic;
extern crate image;
extern crate rand;
mod stroke;
use stroke::Painting;
use rsgenetic::sim::par::Simulator;
use rsgenetic::sim::*;
use rsgenetic::sim::select::*;

fn main() {
    let mut population:Vec<Painting> = (0..5000).map(|_| Painting::informed_random("image.png", 10000)).collect();
    population[0].render_painting("sample.png");
    population[1].render_painting("sample2.png");
    let mut s = Simulator::builder(&mut population)
                    .set_selector(Box::new(MaximizeSelector::new(10)))
                    .set_max_iters(50)
                    .build();
    s.run();
    println!("the most fit member is: {}", s.get().unwrap().fitness());
    s.get().unwrap().render_and_save_image();
    

}
