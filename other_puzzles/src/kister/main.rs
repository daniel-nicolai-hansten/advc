fn main() {
    println!("{}", t(1, 5));
    println!("{}", min_dynamite_cost(2, 10));
}
pub fn min_dynamite_cost(chests: usize, levels: usize) -> usize {
    // Base cases
    if levels == 0 {
        return 0;
    }
    if chests == 0 {
        return usize::MAX / 2; // Can't test without chests
    }
    if chests == 1 {
        // Must test linearly: 1+2+3+...+levels
        return levels * (levels + 1) / 2;
    }

    let mut min_cost = usize::MAX;

    // Try testing at each position h (costs h dynamites)
    for h in 1..=levels {
        // If chest breaks at h: need to check 1..(h-1) with (chests-1)
        let break_cost = match h {
            1 => 0,
            _ => min_dynamite_cost(chests - 1, h - 1),
        };

        // If chest survives at h: we know threshold is > h
        // Need to check (h+1)..levels, but counting from h+1
        // So we need to ADD h to each subsequent test in that branch
        let survive_cost = match h {
            h if h >= levels => 0,
            _ => {
                let remaining = levels - h;
                h + min_dynamite_cost(chests, remaining)
            }
        };

        // Worst case cost for testing at h
        let worst_case = h + break_cost.max(survive_cost);
        min_cost = min_cost.min(worst_case);
    }

    min_cost
}

fn t(chests: u32, levels: u32) -> u32 {
    if levels == 0 {
        return 0;
    }
    if chests == 0 {
        return u32::MAX / 2; // Can't test without chests
    }
    if chests == 1 {
        // Must test linearly: 1+2+3+...+levels
        return levels * (levels + 1) / 2;
    }

    let mut min_cost = u32::MAX;

    // Try testing at each position h (costs h dynamites)
    for h in 1..=levels {
        // If chest breaks at h: need to check 1..(h-1) with (chests-1)
        let break_cost = if h > 1 { t(chests - 1, h - 1) } else { 0 };

        // If chest survives at h: need to check (h+1)..levels with chests
        let survive_cost = if h < levels { t(chests, levels - h) } else { 0 };

        // Worst case cost for testing at h
        let worst_case = h + break_cost.max(survive_cost);
        min_cost = min_cost.min(worst_case);
    }

    min_cost
}
