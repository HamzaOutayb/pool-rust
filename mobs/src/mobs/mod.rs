mod boss;
pub use boss::*;

mod member;
pub use member::*;

use std::collections::{HashMap, HashSet};
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String,Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn recruit(&mut self, m_info: (&str, u32)) {
       self.members.insert(m_info.0.to_string(), Member{
        role: Role::Associate,
        age: m_info.1,
       });
    }

    pub fn attack(&mut self, other: &mut Self) {
        let mut age = u32::MAX;
        let mut name = String::new();
        let mut power = 0;
        for (k, v) in &self.members {
            if age > v.age {
                age = v.age;
                name = k.clone();
            }

           let p = match v.role {
            Role::Underboss => 4,
            Role::Caporegime => 3,
            Role::Soldier => 2,
            Role::Associate => 1,
            };

            power += p;
        }

        let mut age2 = u32::MAX;
        let mut name2 = String::new();
        let mut power2 = 0;
        for (k, v) in &other.members {
            if age2 > v.age {
                age2 = v.age;
                name2 = k.clone();
            }

            let p2 = match v.role {
            Role::Underboss => 4,
            Role::Caporegime => 3,
            Role::Soldier => 2,
            Role::Associate => 1,
            };
            
            power2 += p2;
        }


        if power <= power2 {
            self.members.remove(&name);
            if self.members.len() == 0 {
                other.wealth += self.wealth;
                 other.cities.extend(self.cities.clone());
            }
        } else {
            other.members.remove(&name2);
            if self.members.len() == 0 {
                self.wealth += other.wealth;
                 self.cities.extend(other.cities.clone());
            }
        }
    }

    pub fn steal(&mut self, target: &mut Self, steal: u64) {
        if target.wealth >= steal {
            target.wealth -= steal;
            self.wealth += steal;
        } else {
            self.wealth += target.wealth;
            target.wealth = 0;
        }
    }

    pub fn conquer_city(&mut self, mobs: &[Mob], city: String) {
        let mut have_it = false;
        for mob in mobs {
            if mob.cities.contains(&city) {
                have_it = true;
            }
        }
        if have_it {
            self.cities.insert(city);
        }
    }
}