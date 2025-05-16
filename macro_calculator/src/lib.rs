pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub proteins: f64,
    pub carbs: f64,
    pub fats: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut cals: f64 = 0.0;
    let mut carbs: f64 = 0.0;
    let mut proteins: f64 = 0.0;
    let mut fats: f64 = 0.0;
    for food in foods {
        let kcal = food.calories.1.replace("kcal", "").parse::<f64>().unwrap_or(0.0);
        cals +=  kcal * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
    }

    let cals = (cals*100.0).round()/100.0;
    let carbs = (carbs*100.0).round()/100.0;
    let proteins = (proteins*100.0).round()/100.0;
    let fats = (fats*100.0).round()/100.0;
    let mut res = json::JsonValue::new_object();
    res["cals"] = cals.into();
    res["carbs"] = carbs.into();
    res["proteins"] = proteins.into();
    res["fats"] = fats.into();
    res
}
//format!("{:.2}",cals).parse::<f64>().unwrap()