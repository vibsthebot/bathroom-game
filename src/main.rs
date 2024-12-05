mod random;
use random::render_array as re;
fn main() {
    //println!("{:?}", fixed_array([1, 0, 0, 0, 0, 0, 0, 0, 1].to_vec()));
    let mut level = 1;
    println!("Instructions: X is a filled stall, O is an empty one. Type the number of the stall you should use.");
    loop {
        println!("Level {}", level);
        re(9, 3);
        std::thread::sleep(std::time::Duration::from_secs(1));
        level += 1;
    }
}
