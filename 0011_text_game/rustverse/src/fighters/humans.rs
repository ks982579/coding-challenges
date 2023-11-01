use super::Fighter;

#[derive(Default, Debug, Clone)]
pub enum AttackTypes {
    #[default]
    Physical,
    Energy,
    Fire,
    Electric,
    Ice,
}

pub struct BasicAttack {
    pub attack_types: Vec<AttackTypes>,
    pub damage: u32,
    pub effects: Option<String>,
}

#[derive(Default, Debug, Clone)]
pub struct Human {
    pub name: String,
    pub health: u32,
    pub strength: u32,
}

impl Fighter for Human {
    fn attack<T> (&self, opponent: &mut T)
        where T: Fighter, {
        opponent.take_damage(&self.strength);
    }
    fn take_damage(&mut self, damage: &u32) {
        if self.health >= *damage {
            self.health -= *damage;
        } else {
            self.health = 0;
        }
    }
}