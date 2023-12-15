use std::{io, cmp::Ordering};
use rand::Rng;
// use 키워드를 통해 외부 패키지를 가져올 수 있다.
// rand::Rng는 rand 패키지의 Rng 트레이트를 가져온다.
// cargo  doc --open 명령어를 통해 패키지의 문서를 확인할 수 있다. 
// ex - 어떤 트레이트가 어떤 메서드를 가지고 있는지 확인할 수 있다.


fn main() {
    println!("Guessing the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);


    println!("Please input your guess.");
    
    let mut guess = String::new();
    // mut 키워드를 통해 가변성을 부여한다.
    // Rust에서 기본적으로 변수는 불변성을 가진다.
    
    

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
