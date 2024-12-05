use rand::Rng;

pub fn random_array(x :usize, y :i32) -> Vec<i32> {

    let mut array = vec![0; x]; 

    let mut rng = rand::thread_rng();
    for _ in 0..y {
        let random_index = rng.gen_range(0..x);
        array[random_index] = 1; 
    }

    array
}

pub fn fixed_array(array: Vec<i32>) -> Vec<i32> {
    let mut array = array;
    for i in 0..array.len() {
        if array[i] == 0
            && (i == 0 || (array[i - 1] != 1 && array[i - 1] != 2))
            && (i == array.len() - 1 || (array[i + 1] != 1 && array[i + 1] != 2))
        {
            array[i] = 2;
        }
    }
    array
}

pub fn render_array(x :usize, y :i32) {
    let array = fixed_array(random_array(x, y));
    for &i in &array {
        //print!("{}", i);
        if i == 1 {
            print!("X ");
        } /*else if i == 2 {
            print!("Y ");
        }*/ else {
            print!("O ");
        }
    }
    println!(" ");
    for i in 1..=array.len() {
        print!("{} ", i);
    }
    println!(" ");
    let input = std::io::stdin();
    let mut input_string = String::new();
    input.read_line(&mut input_string).expect("Failed to read line");
    let input_number: usize = input_string.trim().parse().expect("Please type a number!");

    if input_number > 0 && input_number <= array.len() && array[input_number - 1] == 2 {
        println!("Correct!");
    } else {
        println!("Input is not oprimized!");
    }
    println!("All correct answers:");
    for (index, &value) in array.iter().enumerate() {
        if value == 2 {
            print!("{} ", index + 1);
        }
    }
    println!();
}