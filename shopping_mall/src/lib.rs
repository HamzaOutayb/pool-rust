// pub mod mall;
// pub use mall::*;
// use std::collections::HashMap;

// //receives a Mall and returns the Store with the most square_meters.
// pub fn biggest_store(mall: &Mall) -> (String, Store) {
//     let mut store = Store{
//         employees: HashMap::new(),
//         square_meters: 0,
//     };
//     let mut max = 0;
//     let mut store_name: String = String::new();
//    for (_, v) in &mall.floors {
//         for (k, v) in &v.stores {
//             if v.square_meters > max {
//                 max = v.square_meters;
//                 store = v.clone();
//                 store_name = k.clone();
//             }
//         }
//    }
    
//    (store_name, store)
// }

// pub fn highest_paid_employee(mall: &Mall) -> Vec<(&str, Employee)> {
//     let mut highest_paid: Vec<(&str, Employee)> = Default::default();

//     let mut highest: f64 = 0.0;
//     for (_,floor) in &mall.floors {
//         for (_, store) in &floor.stores {
//             for (_, v) in &store.employees {
//                 if v.salary > highest {
//                     highest = v.salary;
//                 }
//             }
//         }
//     }

//         for (_,floor) in &mall.floors {
//         for (_, store) in &floor.stores {
//             for (n, v) in &store.employees {
//                 if v.salary == highest {
//                     highest_paid.push((&n, v.clone()));
//                 }
//             }
//         }
//     }

    

//     highest_paid
// }

// // receives a Mall and returns the number of employees and guards as a usize.
// pub fn nbr_of_employees(mall: &Mall) -> usize {
//     let guards = mall.guards.len() as usize;
//     let mut employees = 0 as usize;
//      for (_,floor) in &mall.floors {
//         for (_, store) in &floor.stores {
//             employees += store.employees.len() as usize;
//         }
//     }

//     guards + employees
// }

// pub fn check_for_securities(mall: &mut Mall, mut unemp: Vec<(String, Guard)>) {
//     let mut floor_size = 0;
//     for (_, v) in mall.floors.clone() {
//         floor_size += v.size_limit;
//     };

//     let guard_needed = floor_size/200;
//     while guard_needed > ((mall.guards.len() as usize)).try_into().unwrap() {
//             let (name, guard) = unemp.pop().unwrap();
//             mall.guards.insert(name.clone(), guard.clone());
//     };
// }

// pub fn cut_or_raise(m: &mut Mall) {
//     for f in m.floors.values_mut() {
//         for s in f.stores.values_mut() {
//             for e in s.employees.values_mut() {
//                 println!("{:?}", e.working_hours);
//                 if e.working_hours.1 - e.working_hours.0 >= 10 {
//                     e.salary += (e.salary / 100 as f64) * 10 as f64
//                 } else {
//                     e.salary -= (e.salary / 100 as f64) * 10 as f64
//                 }
//             }
//         }
//     }
// }

pub mod mall;
pub use mall::*;
use std::collections::HashMap;

pub fn biggest_store(mall: &Mall)-> (String, Store) {
    let mut name: String = String::new();
    let mut store = Store {
        employees: HashMap::new(),
        square_meters: 0,
    };
    let mut max = 0;
    for (_, v) in &mall.floors {
        for (k , v) in &v.stores {
            if v.square_meters > max {
                max = v.square_meters;
                store = v.clone();
                name = k.clone();
            }
        }
    }
    (name, store)
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

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let guards = mall.guards.len() as usize;
    let mut nbr_employee = 0 as usize;
    for (_,v) in &mall.floors {
        for (_, v) in &v.stores {
        nbr_employee += v.employees.len();
        }
    }
    nbr_employee + guards
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

pub fn cut_or_raise(m: &mut Mall) {
    for f in m.floors.values_mut() {
        for s in f.stores.values_mut() {
            for e in s.employees.values_mut() {
                // println!("{:?}", e.working_hours);
                if e.working_hours.1 - e.working_hours.0 >= 10 {
                    e.salary += (e.salary / 100 as f64) * 10 as f64
                } else {
                    e.salary -= (e.salary / 100 as f64) * 10 as f64
                }
            }
        }
    }
}