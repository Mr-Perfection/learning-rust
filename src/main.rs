fn main() {
    // u8, u16, u32, u64, u128
    // let unsigned: u8 = 255;
    // // i8, i16, i32, i64, i128
    // let signed: i8 = -127;
    // let float: f32 = 1.213;
    // println!("{}, {}, {}", unsigned, signed, float);

    // let letter = "c1232";
    // let emoji = "\u{1F600}";
    // let is_true: bool = true;
    // println!("letter {}, emoji {}", letter, emoji);

    // // Arrays
    // let arr: [u8; 3] = [1,2,3];
    // let arr2: [i8; 4] = [-1; 4];
    // println!("{:?} {:?} {}", arr, arr2, arr2.len());

    // // Tuples
    // let tuple: (u8, bool, f32) = (5, false, 2.1);
    // let tuple2 = (3, 5);
    
    // let (a, b, c) = tuple;
    // println!("{} . {} . {}", tuple.0, tuple.1, tuple.2);
    // println!("{} - {} - {}", a, b, c);
    // println!("{:?}", tuple2);
    
    // slice
    // let arr: [u8; 4] = [5, 10, 11, 12];
    // let slice = &arr[1 .. 3];
    // borrowing_slice(arr, slice);

    // String
    // let str: &str = "hello world";
    // let mut string: String = String::from(str);

    // let slice = &string[.. 6];
    // println!("len is  {}", slice.len());
    // string.push_str("! yay");
    // string.push('y');
    // string = string.replace("hello", "hola");
    // println!("{}", string);

    // loops
    // for i in 0..=6 {
    //     println!("{} ", i);
    // }
    
    // let mut i = 0;
    // while i < 4 {
    //     println!("while loop: {}", i);
    //     i+=1;
    //     if i == 3 {
    //         break;
    //     }
    // }

    // Match (switch)
    // for i in 0..10 {
    //     match i {
    //         0 => println!("^_^"),
    //         1 | 2 => println!("^_^ | ^_^"),
    //         3..=6 => println!("=^="),
    //         _ => println!("<3")
    //     }
    // }

    // Structs
    // let name = String::from("Birdy");
    // let bird = Bird { name, attack: 55 };
    // bird.print_name();
    // bird.attack_power();

    // Interfaces - rust does not support inheritance.
    // let bird = Bird { name: "Elephant".to_string(), attack: 55 };
    // bird.print();

    // Enums
    // let a = TestEnum::A;
    // let b = TestEnum::B(7);
    // let c = TestEnum::C(5, 19099);
    // let d = TestEnum::D {x: -12, y: 19};
    
    
    // if let TestEnum::B(x) = b {
    //     println!("b_x: {}", x);
    // }

    // if let TestEnum::C(x,y) = c {
    //     println!("c_x: {} ; c_y:{}", x, y);
    // }
    
    // if let TestEnum::D{x,y} = d {
    //     println!("x: {} ; y:{}", x, y);
    // }

    // Vectors
    // let mut vec1: Vec<i128> = vec![-132, -10000, 300, 10000];
    // vec1.len();
    // vec1[0];
    // vec1.push(777);
    // vec1.remove(0);
    // println!("{:?}", vec1);

    // Option
    let divide1 = divide(100, 5);
    let divide2 = divide(100, 7);

    println!("{:?} unwraps to {}", divide1, divide1.unwrap());
    if divide2.is_some() {
        println!("{:?} unwraps to {}", divide2, divide2.unwrap());
    }

}

enum TestEnum {
    A,
    B(u8),
    C(u8, u128),
    D { x: i32, y: u8},
    
}



// fn borrowing_slice(arr: [u8; 4], slice: &[u8]) {
//     println!("{:?}", arr);
//     println!("{:?}", slice);
//     println!("length is {} ", slice.len());
//     println!("{} {}", slice[0], slice[1]);
// }

// struct Bird {
//     name: String,
//     attack: u64
// }

// impl Bird {
//     fn print_name(&self) {
//         println!("{}", self.name);
//     }

//     fn attack_power(&self) {
//         println!("{}", self.attack);
//     }
// }

// #[derive(Debug)]
// enum AnimalType {
//     Air,
//     Land,
//     Water
// }

// impl Animal for Bird {
//     fn can_fly(&self) -> bool {
//         false
//     }

//     fn print(&self) {
//         println!("can fly: {}", self.can_fly());
//         println!("type: {:?}", self.animal_type());
//     }
// }
// trait Animal {
//     fn can_fly(&self) -> bool;
//     fn animal_type(&self) -> AnimalType {
//         AnimalType::Land
//     }
//     fn print(&self);
// }

fn divide(dividend: u128, divisor: u128) -> Option<u128> {
    if dividend % divisor != 0 {
        None
    } else {
        Some(dividend/divisor)
    }
}