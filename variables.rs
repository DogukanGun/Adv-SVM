fn main() {
    let mut sum: i32 = 10;
    sum = 20;
    println!("{}", sum);

    let sum_as_str: i32 = "42".parse().expect("Not a number");
    println!("{}", sum_as_str);

    let tup: (i32, f64, u8) = (500, 5.5, 1);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("4th index of the array is {}", array[4]);

    print_labeled_measurement(5, 'h');

    if sum > 10 {
        println!("Sum is bigger than 10");
    } else {
        println!("Sum is smaller than 10");
    }

    loop_test();
    while_test();
    forloop_test2();
    forloop_with_reverse();
}

fn while_test() {
    let mut index = 0;
    let a = [10, 20, 30, 40, 50];

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn forloop_test2() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn forloop_with_reverse() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn loop_test() {
    let mut count = 0;
    'counting_loop: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count = {count}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
