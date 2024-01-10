
pub fn optimal_algorithm(data: [u8; 1000]) -> u32 {
    let mut data_set = generate_data_sets(data);
    let mut memory: Vec<DataSet> = vec![];
    let mut page_errors = 0;
    for i in 0..data_set.len() {

        let val = data_set.get(i).unwrap().value;

        // Fill empty memory
       if memory.clone().len() < 10 {
           memory.push(DataSet {
               value: val,
               next_use_in: get_next_use_in(val, i+1, &data_set)
           });
       } else {
            if !is_page_in_memory(val, memory.clone()) {
                let replace_index = get_page_to_replace(memory.clone());
                memory[replace_index] = DataSet {
                    value: val,
                    next_use_in: get_next_use_in(val, i+1, &data_set)
                };
                page_errors += 1;
            }
       }
    }
    return page_errors;
}

#[derive(Copy, Clone)]
struct DataSet {
    value: u8,
    next_use_in: u32
}

fn generate_data_sets(data: [u8; 1000]) -> Vec<DataSet> {
    let mut set = vec![];
    for i in 0..data.len() {
        set.push(DataSet {
            value: data[i],
            next_use_in: 0,
        });
    }
    return set;
}

fn get_next_use_in(num: u8, offset: usize, data: &Vec<DataSet>) -> u32 {
    for i in offset..data.len() {
        if data.get(i).unwrap().value == num {
            return (i - offset) as u32;
        }
    }
    return 0;
}

fn is_page_in_memory(page: u8, memory: Vec<DataSet>) -> bool {
    let values = memory.iter().map(|x|x.value).collect::<Vec<u8>>();
    values.contains(&page)
}

fn get_page_to_replace(memory: Vec<DataSet>) -> usize {
    let mut index = 0;
    let mut highest = 0;
    for i in 0..memory.len() {
        let val = memory.get(i).unwrap().value;
        if val > highest {
            index = i;
            highest = val;
        }
    }
    return index;
}