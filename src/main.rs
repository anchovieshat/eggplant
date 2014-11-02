use graphics::loader::Wavefront;
mod graphics;

fn main() {
    let obj = Wavefront::open(&Path::new("/tmp/derp.obj/untitled.obj"));
    println!("{}", obj);
}
