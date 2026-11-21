mod shoot;
use shoot::Shoot;

fn main() {
    let test: Result<Shoot, &str> = Shoot::new(2.1, 4.5, 7.24, 3.048);

    match test {
        Ok(shoot) => {
            println!("Rising time : {:.2} s", shoot.get_rising_time());
            println!("Falling time : {:.2} s", shoot.get_falling_time());
            println!("Horizontal speed : {:.2} m/s", shoot.get_vx());
            println!("Vertical speed : {:.2} m/s", shoot.get_vy());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
