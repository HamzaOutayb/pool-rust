pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    if n == 1_000_000 {
        return "one million".to_string();
    }

    let units = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let teens = [
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen",
        "sixteen", "seventeen", "eighteen", "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty",
        "sixty", "seventy", "eighty", "ninety",
    ];

    let spell_under_1000 = |n: u64| -> Vec<String> {
        let mut res = Vec::new();

        if n >= 100 {
            res.push(units[(n / 100) as usize].to_string());
            res.push("hundred".to_string());
        }

        let n = n % 100;
        if n >= 20 {
            let mut t = tens[(n / 10) as usize].to_string();
            if n % 10 != 0 {
                t.push('-');
                t.push_str(units[(n % 10) as usize]);
            }
            res.push(t);
        } else if n >= 10 {
            res.push(teens[(n - 10) as usize].to_string());
        } else if n > 0 {
            res.push(units[n as usize].to_string());
        }

        res
    };

    let mut result = Vec::new();

    if n >= 1000 {
        result.extend(spell_under_1000(n / 1000));
        result.push("thousand".to_string());
    }

    let rem = n % 1000;
    if rem != 0 {
        result.extend(spell_under_1000(rem));
    }

    result.join(" ")
}
