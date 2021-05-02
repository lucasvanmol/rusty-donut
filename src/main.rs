use donut::{RayMarcher, Config, viewport_sizes};
use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut vp_size = viewport_sizes::NORMAL;
   

    if args.contains(&String::from("--tiny")) {
        vp_size = viewport_sizes::TINY;
    } else if args.contains(&String::from("--small")) {
        vp_size = viewport_sizes::SMALL;
    } else if args.contains(&String::from("--big")) {
        vp_size = viewport_sizes::BIG;
    }

    let config = Config::new(vp_size, args.contains(&String::from("--hd")));
    
    let mut d = RayMarcher::new(config);
    d.run().expect("An error occured");
}