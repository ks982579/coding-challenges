// use crate::fighters;
pub mod humans;

// use crate::fighters;

pub trait Fighter {
    fn attack<T> (&self, opponent: &mut T)
    where T: Fighter,
    {
        println!("Attacking");
    }
    fn take_damage(&mut self, damage: &u32) {
        println!("Taking Damage!")
    }
}