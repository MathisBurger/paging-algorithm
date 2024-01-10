pub fn last_recently_used_algorithm(data: [u8; 1000]) -> u32 {
    let mut data_set = generate_data_sets(data);
    let mut memory: Vec<DataSet> = vec![];
    let mut page_errors = 0;
    for i in 0..data_set.len() {

        let val = data_set.get(i).unwrap().value;

        // Fill empty memory
        if memory.clone().len() < 10 {
            memory.push(DataSet {
                value: val,
                last_used: i as u32
            });
        } else {
            if !is_page_in_memory(val, memory.clone()) {
                let replace_index = get_page_to_replace(memory.clone());
                memory[replace_index] = DataSet {
                    value: val,
                    last_used: i as u32
                };
                page_errors += 1;
            } else {
                let index = find_memory_index(val, memory.clone());
                memory[index] = DataSet {
                    value: memory[index].value,
                    last_used: i as u32
                };
            }
        }
    }
    return page_errors;
}

#[derive(Copy, Clone)]
struct DataSet {
    value: u8,
    last_used: u32
}

fn generate_data_sets(data: [u8; 1000]) -> Vec<DataSet> {
    let mut set = vec![];
    for i in 0..data.len() {
        set.push(DataSet {
            value: data[i],
            last_used: 0,
        });
    }
    return set;
}

fn is_page_in_memory(page: u8, memory: Vec<DataSet>) -> bool {
    let values = memory.iter().map(|x|x.value).collect::<Vec<u8>>();
    values.contains(&page)
}

fn find_memory_index(page: u8, memory: Vec<DataSet>) -> usize {
    let values = memory.iter().map(|x|x.value).collect::<Vec<u8>>();
    for i in 0..values.len() {
        if values[i] == page {
            return i;
        }
    }
    return 0;
}

fn get_page_to_replace(memory: Vec<DataSet>) -> usize {
    let mut index = 0;
    let mut lowest = 0xFF;
    for i in 0..memory.len() {
        let val = memory.get(i).unwrap().value;
        if val < lowest {
            index = i;
            lowest = val;
        }
    }
    return index;
}