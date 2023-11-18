fn main() {
    println!("=== Bubble Sort ===");
    
    let mut my_vector: Vec<f32> = vec![
        12.5, 6.8, 23.0, 45.2, 8.7, 33.1, 17.9, 5.4, 29.8, 10.3,
        42.6, 14.2, 7.6, 19.4, 37.0, 3.2, 28.6, 9.1, 21.8, 49.5,
        31.7, 15.9, 4.5, 26.3, 11.0, 40.1, 22.7, 2.0, 35.4, 18.3,
    ];


    sort(&mut my_vector, "bubble");
    
    println!("\nO vetor com sorted é:\n{:?}", my_vector);
}

fn sort(received_vector : &mut Vec<f32> , sort_method : &str) {
    if sort_method == "bubble" {
        bubble_sort(received_vector);
    }
}

fn bubble_sort(received_vector: &mut Vec<f32>) -> Vec<f32> {
    let mut swapped:bool ;

    for i in 0..(received_vector.len() - 1) {
        swapped = false;

        for j in 0..(received_vector.len() - i - 1) {
            if received_vector[j] > received_vector[j + 1] {
                let temp = received_vector[j];
                received_vector[j] = received_vector[j + 1];
                received_vector[j + 1] = temp;

                swapped = true;
            }
        }

        if !swapped {
            return received_vector.clone();
        }
    }
    
    return received_vector.clone()
}

/* 
fn get_sort(mut vector: Vec<f32>, sort_method: &str) -> Vec<f32> {
    if sort_method == "bubble" {
        return bubble_sort( &mut vector);
    } else {
        return vec![0.0];
    }
}

fn bubble_sort(received_vector: &mut Vec<f32>) -> Vec<f32> {
    let mut swapped:bool ;

    for i in 0..(received_vector.len() - 1) {
        swapped = false;

        for j in 0..(received_vector.len() - i - 1) {
            if received_vector[j] > received_vector[j + 1] {
                let temp = received_vector[j];
                received_vector[j] = received_vector[j + 1];
                received_vector[j + 1] = temp;

                swapped = true;
            }
        }

        if !swapped {
            return received_vector.clone();
        }
    }
    
    return received_vector.clone()
}
*/
