mod shoot;
use shoot::Shoot;

fn main() {
    let test: Shoot = Shoot::new(2.1, 4.5, 7.24, 3.048);

    println!("Rising time : {:.2} s", test.get_rising_time());
    println!("Falling time : {:.2} s", test.get_falling_time());
    println!("Horizontal speed : {:.2} m/s", test.get_vx());
    println!("Vertical speed : {:.2} m/s", test.get_vy());
}
