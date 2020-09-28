pub fn map_num(n: f32, min_old: f32, max_old: f32, min_new: f32, max_new: f32) -> f32 {
    if n < min_old {
        return min_new;
    
    } else if n > max_old {
        return max_new;
    } else {
        return (((n - min_old) / (max_old - min_old)) * (max_new - min_new)) + min_new;
    }
}