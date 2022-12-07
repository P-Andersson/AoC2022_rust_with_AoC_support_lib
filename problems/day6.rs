use std::collections::HashSet;

pub fn parser(raw_input: &String) -> String {
    return raw_input.clone();
}

pub fn solverP1(input: String) -> String {
    let data = input.as_bytes();
    for end_index in 4..data.len()+1 {
        let mut found: HashSet<u8>  = HashSet::new();
        for i in end_index-4..end_index {
            found.insert(data[i]);
        }
        if found.len() == 4 {
            return end_index.to_string();
        }
    }

    return input.len().to_string();
}

pub fn solverP2(input: String) -> String {
    let data = input.as_bytes();
    for end_index in 14..data.len()+1 {
        let mut found: HashSet<u8>  = HashSet::new();
        for i in end_index-14..end_index {
            found.insert(data[i]);
        }
        if found.len() == 14 {
            return end_index.to_string();
        }
    }

    return input.len().to_string();
}
