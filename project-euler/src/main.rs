mod euler1;
mod euler2;
mod euler3;
mod euler7;
mod euler35;

fn main() {
    let answer_1 = euler1::euler_1();
    println!("{}", answer_1);

    let answer_2 = euler2::euler_2();
    println!("{}", answer_2);

    let answer_3 = euler3::euler_3();
    println!("{}", answer_3);

    let answer_7 = euler7::euler_7();
    println!("{}", answer_7);

    let answer_35 = euler35::euler_35();
    println!("{}", answer_35);
}

