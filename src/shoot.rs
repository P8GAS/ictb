use getset::{Getters, Setters};

const G: f32 = 9.81; // acceleration due to gravity in m/s^2

#[derive(Getters, Setters)]
pub struct Shoot {
    #[getset(get, set)]
    start_height: f32, // starting height of the ball's center
    #[getset(get, set)]
    highest_point: f32, // highest point of the arc followed by the ball's center
    #[getset(get, set)]
    basket_distance: f32, // horizontal distance between the basket's center and the ball's center
    #[getset(get, set)]
    basket_height: f32, // height of the basket
}

impl Shoot {
    pub fn new(sh: f32, hp: f32, bd: f32, bh: f32) -> Result<Shoot, &'static str> {
        if sh <= 1.0 || sh > hp || bh <= 0.0 { 
            return Err("Invalid parameters: start_height must be > 1.0 and <= highest_point, basket_height must be > 0.0"); 
        }
        
        Ok(Shoot { 
            start_height: sh,
            highest_point: hp,
            basket_distance: bd,
            basket_height: bh 
        })
    }

    pub fn get_rising_time(&self) -> f32 {
        ((2.0 * (self.highest_point() - self.start_height())) / G).sqrt()
    }

    pub fn get_falling_time(&self) -> f32 {
        ((2.0 * (self.highest_point() - self.basket_height())) / G).sqrt()
    }

    pub fn get_vx(&self) -> f32 {
        self.basket_distance() / (self.get_falling_time() + self.get_rising_time())
    }

    pub fn get_vy(&self) -> f32 {
        G * self.get_rising_time()
    }
}