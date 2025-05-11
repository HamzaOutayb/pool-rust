use std::usize;

fn edite(source: &str, target: &str, s_len: usize, t_len: usize) -> usize {
    if s_len == 0 {
        return t_len
    }
    if t_len == 0 {
        return s_len
    }

    if source.chars().nth(s_len-1) == target.chars().nth(t_len-1) {
        return edite(source, target, s_len-1, t_len-1)
    }

    let df: usize = min(
    [edite(source, target, s_len, t_len-1),
         edite(source, target, s_len-1, t_len),
         edite(source, target, s_len-1, t_len-1),]
    );

    return 1+df
}

fn min(arr: [usize; 3]) -> usize {
    let mut min: usize = arr[1];
    for n in arr {
        if min > n {
            min = n
        }
    }
    min
} 

pub fn edit_distance(source: &str, target: &str) -> usize {
    return edite(source, target, source.len(), target.len())
}