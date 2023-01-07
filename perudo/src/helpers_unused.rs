struct HelpersUnused {}

impl HelpersUnused
{
    pub fn cap_guess_num(num_dice:usize, num:usize) -> usize
    {
        if num >= num_dice { return num_dice; }
        if num <= 1 { return 1; }
        return num;
    }

    pub fn cap_guess_val(val:usize) -> usize
    {
        if val >= 6 { return 6; }
        if val <= 1 { return 1; }
        return val;
    }

    pub fn get_highest_val(dice:&Vec<usize>) -> usize
    {
        return *dice.iter().max().unwrap();
    }

    pub fn get_lowest_val(dice:&Vec<usize>) -> usize
    {
        return *dice.iter().min().unwrap();
    }

    pub fn get_mid_val(dice:&Vec<usize>) -> usize
    {
        let max = Helpers::get_highest_val(dice);
        let min = Helpers::get_lowest_val(dice);
        for v in dice.iter()
        {
            if *v == min || *v == max { continue; }
            return *v;
        }
        return max;
    }

    pub fn get_most_frequent_val(dice:&Vec<usize>) -> usize
    {
        let vec = Helpers::sort_by_frequency(dice);
        return vec[vec.len() - 1];
    }

    pub fn get_mid_frequent_val(dice:&Vec<usize>) -> usize
    {
        let vec = Helpers::sort_by_frequency(dice);
        let avg_idx = (0.5 * (vec.len() as f64)).floor() as usize;
        return vec[avg_idx];
    }

    pub fn get_least_frequent_val(dice:&Vec<usize>) -> usize
    {
        let vec = Helpers::sort_by_frequency(dice);
        return vec[0];
    }

    pub fn get_closest_val_ascending(dice:&Vec<usize>, val:usize) -> usize
    {
        let mut closest_dist:i32 = 1000;
        let mut closest_val:usize = 0;
        for v in dice.iter()
        {
            let dist:i32 = (*v as i32) - (val as i32);
            if dist < 0 { continue; }
            if dist >= closest_dist { continue; }

            closest_dist = dist;
            closest_val = *v;
        }
        return closest_val;
    }

    // This sorts ASCENDING
    pub fn sort_by_frequency(dice:&Vec<usize>) -> Vec<usize>
    {
        let mut m: HashMap<usize, usize> = HashMap::new();
        for v in dice {
            *m.entry(*v).or_default() += 1;
        }

        let mut arr:Vec<usize> = m.clone().into_keys().collect();
        arr.sort_by(|a, b| m[a].partial_cmp(&m[b]).unwrap());

        return arr.clone();
    }
}