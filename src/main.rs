use donut::{RayMarcher, Config, viewport_sizes};
use std::{env, process};

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut vp_size = viewport_sizes::NORMAL;
   
    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        println!("\
Usage: donut [options]
    options:
        -h, --help  shows this help message
        --sd        use 10 character charset (default 70)
    
    options (sizes, default is normal size):
        --tiny      shows tiny donut
        --small     shows small donut
        --big       shows big donut
        --huge      shows huge donut");
        process::exit(0);
    }

    if args.contains(&String::from("--tiny")) {
        vp_size = viewport_sizes::TINY;
    } else if args.contains(&String::from("--small")) {
        vp_size = viewport_sizes::SMALL;
    } else if args.contains(&String::from("--big")) {
        vp_size = viewport_sizes::BIG;
    } else if args.contains(&String::from("--huge")) {
        vp_size = viewport_sizes::HUGE;
    }

    let config = Config::new(vp_size, !args.contains(&String::from("--sd")));
    
    let mut d = RayMarcher::new(config);
    d.run().expect("An error occured");
}