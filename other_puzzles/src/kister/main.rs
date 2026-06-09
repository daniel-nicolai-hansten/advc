use cached::proc_macro::cached;

fn main() {
    println!("{}", min_dynamite_cost(2, 17));
    
    println!("{}", min_dynamite_cost(4, 44));
    
    println!("{}", min_dynamite_cost(2, 89));
    
    println!("{}", min_dynamite_cost(2, 9));
}
pub fn min_dynamite_cost(chests: usize, levels: usize) -> usize {
    min_cost_range(chests, 1, levels)
}

#[cached]
fn min_cost_range(chests: usize, low: usize, high: usize) -> usize {
    if low > high {
        return 0;
    }
    if chests == 0 {
        return usize::MAX / 2;
    }
    if chests == 1 {
        // Must test linearly: low + (low+1) + ... + high
        let n = high - low + 1;
        return n * (low + high) / 2;
    }

    let mut min_cost = usize::MAX;

    for h in low..=high {
        // Costs h dynamite to test at level h
        let break_cost = if h > low {
            min_cost_range(chests - 1, low, h - 1)
        } else {
            0
        };
        let survive_cost = if h < high {
            min_cost_range(chests, h + 1, high)
        } else {
            0
        };

        let worst_case = h + break_cost.max(survive_cost);
        min_cost = min_cost.min(worst_case);
    }

    min_cost
}


