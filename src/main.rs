
fn get_max(arr: &mut Vec<usize>) -> usize {
    let mut temp: usize = arr[0];
    for i in 1..arr.len() {
        if temp < arr[i] {
            temp = arr[i];
        }
    }
    return temp;
}

fn get_min(arr: &mut Vec<usize>) -> usize {
    let mut temp: usize = arr[0];
    for i in 1..arr.len() {
        if temp > arr[i] {
            temp = arr[i];
        }
    }
    return temp;
}

fn sort(arr: &mut Vec<usize>) {
    let min: usize = get_min(arr);
    let max: usize  = get_max(arr);
    let mut z: usize = 0;

    let mut count = vec![0; max - min +1];

    for i in 0..arr.len() {
        count[arr[i] - min] = count[arr[i] - min] + 1
    }

    for i in min..max+1 {
        while count[i - min] > 0 {
            arr[z] = i;
            z += 1;
            count[i - min] = count[i - min] - 1;
        }
    }
}

fn main() {
    let mut array: Vec<usize> = vec![20, 9, 5, 2, 4, 6, 1, 3, 400, 50];

    println!("Original Array: {:?}", array);

    sort(&mut array);

    println!("Sorted Array: {:?}", array);
}