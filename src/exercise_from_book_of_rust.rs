use std::collections::HashMap;

fn mean(v: & Vec<i32>) -> f32 {
    let len: i32 = v.len() as i32;
    let sum: i32 = v.iter().sum();
    let mean = sum as f32 / len as f32;

    return mean;
}

fn median(v: &mut Vec<i32>) -> i32 {
    v.sort();
    let len: usize = v.len();

    if len % 2 == 0 {
        return v[len/2];
    }
    else {
        let ind: usize = (len as f32 / 2.0).ceil() as usize;
        return v[ind]
    }

}

fn mode(v: & Vec<i32>)-> Option<i32> {
    let mut map: HashMap<String, i32> = HashMap::new();

    for el in v{
        let count = map.entry(el.to_string()).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let mut num: Option<i32> = None;
    let mut mode: i32 = 0;

    for (key, val) in map {
        if val > mode {
            mode = val;
            num = Some(key.parse::<i32>().unwrap());
        }
    }

    return num;
}


fn main() {
    let mut vect = vec![1, 5, 5, 3, 5, 6, 2, 3, 4, 1, 7];

    let mean = mean(&vect);
    let median = median(&mut vect);
    let mode = mode(&vect);

    println!("Mean: {:?}", mean);
    println!("Median: {:?}", median);
    if let Some(value) = mode {
        println!("Mode: {:?}", value);
    }
    else {
        println!("No values in vector!");
    }

}