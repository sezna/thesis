extern crate palette;
extern crate imageproc;
extern crate rsgenetic;
extern crate image;
extern crate clap;
extern crate rand;
mod stroke;
use stroke::Painting;
use rsgenetic::sim::par::Simulator;
use rsgenetic::sim::*;
use rsgenetic::sim::select::*;
use clap::{Arg, App};

fn main() {
    let args = App::new("Genetic Painter")
                   .version("1.0")
                   .author("Alexander Hansen <alex@alex-hansen.com>")
                   .about("Paints a picture")
                   .arg(Arg::with_name("size")
                            .short("s")
                            .long("size")
                            .value_name("POPSIZE")
                            .help("Sets the size of the initial population")
                            .takes_value(true)
                            .required(true))
                   .arg(Arg::with_name("numstrokes")
                            .short("n")
                            .long("numstrokes")
                            .value_name("NUMSTROKES")
                            .help("Sets the number of strokes per painting in the population")
                            .takes_value(true)
                            .required(true))
                   .arg(Arg::with_name("imagefile")
                            .short("f")
                            .long("file")
                            .value_name("IMAGEFILE")
                            .help("Selects the image file to use for painting")
                            .takes_value(true)
                            .required(true))
                   .arg(Arg::with_name("iterations")
                            .short("i")
                            .long("iterations")
                            .value_name("NUMITER")
                            .help("Sets the maximum number of iterations when running the \
                                   genetic algorithm")
                            .takes_value(true))
                   .arg(Arg::with_name("verbose")
                            .short("v")
                            .long("verbose")
                            .help("Sets the verbosity level from 0 to 2")
			    .takes_value(true))
                   .arg(Arg::with_name("random")
                            .short("r")
                            .long("random")
                            .help("Sets the generation of the initial population to be random \
                                   instead of \"informed\""))
                   .arg(Arg::with_name("strokewidth")
                            .short("w")
                            .long("strokewidth")
                            .help("Sets the maximum width of strokes.")
                            .takes_value(true))
                   .get_matches();
    // Required args.
    let size: u32 = args.value_of("size").unwrap().parse().unwrap();
    let number_of_strokes: u32 = args.value_of("numstrokes").unwrap().parse().unwrap();
    let image_file = args.value_of("imagefile").unwrap();

    // Optional args.
    let iterations: u64 = args.value_of("iterations").unwrap_or("100").parse().unwrap();
    let verbosity: u32 = args.value_of("verbose").unwrap_or("0").parse().unwrap();
    println!("verbosity set to {}", verbosity);
    let random_generation: bool = args.is_present("random");
    let width: u32 = args.value_of("strokewidth").unwrap_or("5").parse().unwrap();

    println!("{}",
             match verbosity {
                 0 => String::new(),
                 _ => format!("Parameters: \n size: {} \nnumber of strokes: {}\n image_file: {}\n
                   iterations: {}\n random generation is {}\n stroke width: {} ", size,
                   number_of_strokes, image_file, iterations, random_generation, width),
             });

    let mut population: Vec<Painting> = (0..size)
                                            .map(|_| {
                                                if random_generation {
                                                    Painting::random(image_file,
                                                                     number_of_strokes,
                                                                     width)
                                                } else {
                                                    Painting::informed_random(image_file,
                                                                              number_of_strokes,
                                                                              width)
                                                }
                                            })
                                            .collect();
    if verbosity == 2 {
        println!("{} paintings added", population.len());
        println!("Now saving two sample images from the original population");
    }
    population[0].render_painting("sample.png");
    population[1].render_painting("sample2.png");
    let mut s = Simulator::builder(&mut population)
                    .set_selector(Box::new(StochasticSelector::new(10)))
                    .set_max_iters(iterations)
                    .build();
    s.run();
    println!("the most fit member is: {}", s.get().unwrap().fitness());
    s.get().unwrap().render_and_save_image();
}
