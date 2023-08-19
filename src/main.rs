fn main() {
    loop {
        let mut num = String::new();

        std::io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line!");

        let num: u64 = match num.trim().parse() {
            Ok(num) => num,
            Err(_e) => {
                println!("Please input a valid number!");
                continue;
            }
        };

        let _fib_num = gen_fib(num);
        let fib_num = gen_fib_while(num);

        println!("Fibonacci number {num} is {fib_num}");
    }
}

//Using for loop
fn gen_fib(n: u64) -> u64 {
    let mut prev = 0;
    let mut current = 1;
    let mut next;

    if n <= 1 {
        return n;
    }

    for _i in 1..n {
        next = prev + current;
        prev = current;
        current = next;
    }
    return current;
}

//Using while loop
fn gen_fib_while(n: u64) -> u64 {
    let mut n = n;

    let mut prev = 0;
    let mut current = 1;
    let mut next;

    if n <= 1 {
        return n;
    }

    while n > 1 {
        next = prev + current;
        prev = current;
        current = next;
        n -= 1;
    }
    return current;
}

//Using the recursion approach (is very slow and makes my fans spin lol)
// fn gen_fib_alternate(n: u64) -> u64 {
//     if n <= 1 {
//         return n;
//     };
//
//     return gen_fib_alternate(n - 1) + gen_fib_alternate(n - 2);
// }
