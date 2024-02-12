use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Progress {
    None,
    Some,
    Complete,
}

pub fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if val == &value {
            count += 1;
        }
    }
    count
}

pub fn count(map: &HashMap<String, Progress>, value: Progress) -> usize {
    map.values()
        .into_iter()
        .filter(|&progress|value == *progress)
        .count()
}

pub fn count_stack_for(stack: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in stack {
        for val in map.values() {
            if val == &value {
                count += 1;
            }
        }
    }
    count
}

pub fn count_stack(stack: &[HashMap<String, Progress>], value: Progress) -> usize {
    stack.into_iter()
        .flat_map(|hash_map| hash_map.values())
        .filter(|&progress|*progress == value)
        .count()
}