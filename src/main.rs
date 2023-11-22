

fn getA() -> Vec<Vec<u8>>{
    vec![
        vec![1,0,0,0],
        vec![1,1,0,0],
        vec![1,1,1,0],
        vec![1,1,1,1],
        vec![0,1,1,1],
        vec![0,0,1,1],
        vec![0,0,0,1],
    ]
}

fn add_modulo2(x: u8, y: u8) -> u8 {
    let res = x + y;
    if res == 2 {0} else {res}
}

fn multiply_modulo2(x: u8, y: u8) -> u8{
    let res = x & y;
    return res;
}

/// expects binary 7 element vector and 7row,4column element matrix
fn multiply_vector_by_matrix(vector: &Vec<u8>, matrix: Vec<Vec<u8>>) -> Vec<u8>{
    let mut res: Vec<u8> = vec![];

    for i in 0..4{
        let mut elem = 0;
        for j in 0..7{
            elem = add_modulo2(elem, multiply_modulo2(vector[j],matrix[j][i]));
        }
        res.push(elem);
    }
    return res;
}

fn hx(x: &Vec<u8>) -> Vec<u8>{
    multiply_vector_by_matrix(x, getA())
}


fn functional_push_right(vec: Vec<u8>, value: u8) -> Vec<u8> {
    // vec.into_iter().chain([value].into_iter()).collect()
    let mut vec_clone = vec.clone();
    vec_clone.push(value);
    vec_clone
}



// expects max value 127
fn create_binary(value: u8, bits_count: i32) -> Vec<u8>{
    fn create_hex_binary_rec(hex_value: u8, counter: i32, acc: Vec<u8>) -> Vec<u8>{
        if counter < 0 {
            return acc;
        }
        let two_value = (2 as u32).pow(counter as u32) as u8;
        let new_counter = counter - 1;
        if hex_value >= two_value{
            let new_acc = functional_push_right(acc, 1);
            create_hex_binary_rec(hex_value - two_value, new_counter, new_acc)
        }else{
            let new_acc = functional_push_right(acc, 0);
            create_hex_binary_rec(hex_value, new_counter, new_acc)
        }
    }
    create_hex_binary_rec(value, bits_count - 1, vec![])
}

fn binary_to_value(binary: &Vec<u8>) -> u32{
    let mut count = binary.len();
    let mut res = 0;
    for bit in binary {
        let multipier = (2 as u32).pow( (count - 1) as u32);
        res += (*bit as u32) * multipier;
        count -= 1;
    }
    return res;
    // 0
}

fn main() {
    // let example_input = vec![0,0,0,0,1,1,1];
    // let res = hx(&example_input);
    // println!("example_input: {:?}", &example_input);
    // println!("res: {:?}", res);

    let expected = vec![0,1,0,1];
    let max = (2 as u32).pow(7) - 1;
    let mut przeciwobrazy = vec![];
    for i in 0..=max{
        let i_as_binary = create_binary(i as u8, 7);
        let res = hx(&i_as_binary);
        if res == expected {
            przeciwobrazy.push(i_as_binary);
        }
    }
    println!("przeciwobrazy:");
    for przeciwobraz in przeciwobrazy{
        let value = binary_to_value(&przeciwobraz);
        println!("{:0>3?} - {:?}  - {:?}",value, przeciwobraz, expected)

    }



    println!("bye");
}
