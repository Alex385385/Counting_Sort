
fn get_max(arr: &mut Vec<usize>) -> usize {
    let mut temp = arr[0];
    for i in 1..arr.len() {
        if temp < arr[i] {
            temp = arr[i];
        }
    }
    return temp;
}

fn get_min(arr: &mut Vec<usize>) -> usize {
    let mut temp = arr[0];
    for i in 1..arr.len() {
        if temp > arr[i] {
            temp = arr[i];
        }
    }
    return temp;
}

fn sort(arr: &mut Vec<usize>) {
    let mut min: usize = get_min(arr);
    let mut max: usize  = get_max(arr);
    let mut z = 0;

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

    let i = 0;
}

/*
function countingSort(array, min, max):
    count: array of (max - min + 1) elements
    initialize count with 0
    for each number in array do
        count[number - min] := count[number - min] + 1
    done
    z := 0
    for i from min to max do
        while ( count[i - min] > 0 ) do
            array[z] := i
            z := z+1
            count[i - min] := count[i - min] - 1
        done
    done
 */

fn main() {
    let mut array = vec![5, 2, 4, 6, 1, 3];
    let sum = 1;

    let sum2: usize = array[0];

    sort(&mut array);
    println!("{}", array[0]);
}