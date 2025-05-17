pub mod mall;
pub use mall::*;
use std::collections::HashMap;

//receives a Mall and returns the Store with the most square_meters.
pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut store = Store{
        employees: HashMap::new(),
        square_meters: 0,
    };
    let mut max = 0;
    let mut store_name: String = String::new();
   for (_, v) in &mall.floors {
        for (k, v) in &v.stores {
            if v.square_meters > max {
                max = v.square_meters;
                store = v.clone();
                store_name = k.clone();
            }
        }
   }
    
   (store_name, store)
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&str, Employee)> {
    let mut highest_paid: Vec<(&str, Employee)> = Default::default();

    let mut highest: f64 = 0.0;
    for (_,floor) in &mall.floors {
        for (_, store) in &floor.stores {
            for (_, v) in &store.employees {
                if v.salary > highest {
                    highest = v.salary;
                }
            }
        }
    }

        for (_,floor) in &mall.floors {
        for (_, store) in &floor.stores {
            for (n, v) in &store.employees {
                if v.salary == highest {
                    highest_paid.push((&n, v.clone()));
                }
            }
        }
    }

    

    highest_paid
}

// receives a Mall and returns the number of employees and guards as a usize.
pub fn nbr_of_employees(mall: &Mall) -> usize {
    let guards = mall.guards.len() as usize;
    let mut employees = 0 as usize;
     for (_,floor) in &mall.floors {
        for (_, store) in &floor.stores {
            employees += store.employees.len() as usize;
        }
    }

    guards + employees
}

pub fn check_for_securities(mall: &mut Mall, mut unemp: Vec<(String, Guard)>) {
    let mut floor_size = 0;
    for (_, v) in mall.floors.clone() {
        floor_size += v.size_limit;
    };

    let guard_needed = floor_size/200;
    while guard_needed > ((mall.guards.len() as usize)).try_into().unwrap() {
            let (name, guard) = unemp.pop().unwrap();
            mall.guards.insert(name.clone(), guard.clone());
    };
}

pub fn cut_or_raise(mall: &mut Mall) {
    for s in mall.floors.values_mut() {
        for v in s.stores.values_mut() {
            for e in v.employees.values_mut() {
                if is_over(e.working_hours) {
                    e.salary = round_to_two_decimal_places(e.salary*1.10);
                } else {
                    e.salary = round_to_two_decimal_places(e.salary*0.90);
                }
            }
        }
    }
}

pub fn is_over(t: (u32, u32)) -> bool {
    let duration = t.1 - t.0;
    duration >= 10
}
fn round_to_two_decimal_places(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}