use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guessing the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);


    println!("Please input your guess.");
    
    let mut guess = String::new();
    
    

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess : u32  = guess
                            .trim()
                            .parse()
                            .expect("Please type a number!");
    // shadowing을 지원한다 -> 변수 재사용해서 덮어 씌우기
    // parse의 뱐환값은 Result 타입이다. Result 타입은 Ok와 Err를 가지고 있다.
    // expect는 Result 타입의 값이 Err일 경우 프로그램을 종료하고, Ok일 경우 Ok의 값을 반환한다.
    // 그래서 parse까지만 작성하면, ide에서는 에러가 발생한다.

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number){
        // cmp는 Ordering 타입을 반환한다.
        // Ordering 타입은 Less, Greater, Equal 세가지 값을 가진다.
        // match는 패턴 매칭을 통해 각각의 경우에 대한 처리를 할 수 있다.
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

}
