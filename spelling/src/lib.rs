pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    if n == 1000000 {
        return "one million".to_string()
    }

    let units = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let teens = [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];


    let func = |n| {
        let mut res = vec![];
        if n < 10 {
            res.push(units[n as usize].to_string());
        } else if n < 20 {
            res.push(teens[n as usize - 10].to_string());
        } else if n < 100 {
            res.push(tens[n as usize / 10].to_string() + "-" + units[n as usize % 10]);
        } else {
            res.push(units[n as usize / 100].to_string());
            res.push("hundred".to_string());
            res.push(tens[n as usize / 100].to_string() + "-" + units[n as usize % 10]);
        }
        res
    };

    
    let mut res = func(n/1000);
    if n >= 1000{
        res.push("thousand".to_string());
    }
    res.extend(func(n%1000));

    res.join(" ").trim().to_string()
}
