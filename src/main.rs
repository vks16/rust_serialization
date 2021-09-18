use serde::{Serialize, Deserialize};
use num::bigint::ToBigInt;
use num_bigint::BigInt;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    // let x = 5i32;
    // let xs = serde_json::to_string(&x).unwrap();
    // println!("i32 number {} serializes into string {}", x, xs);
    // let xd: i32 = serde_json::from_str(&xs).unwrap();
    // assert_eq!(x, xd);

    // let p: Point = Point {x : 0, y: -3};
    // let ps = serde_json::to_string(&p).unwrap();
    // println!("Point {:?} serialize to {}", p, ps);
    // let pd : Point = serde_json::from_str(&ps).unwrap();
    // assert_eq!(p, pd);

    // let x = 5i32;
    // let xs : Vec<u8> = bincode::serialize(&x).unwrap();
    // println!("i32 number {} serializes into byte array {:?}", x, xs);
    // let xd: i32 = bincode::deserialize(&xs).unwrap();
    // assert_eq!(x, xd);

    // let p: Point = Point {x : 0, y: -3};
    // let ps = bincode::serialize(&p).unwrap();
    // println!("Point {:?} serialize to {:?}", p, ps);
    // let pd : Point = bincode::deserialize(&ps).unwrap();
    // assert_eq!(p, pd);

    let a = 3.to_bigint().unwrap();
    let x = num::pow(a, 247);
    let xs = serde_json::to_string(&x).unwrap();
    let xd: BigInt = serde_json::from_str(&xs).unwrap();
    assert_eq!(x, xd);
    println!("3**247 is {} and serializes to {}", x, xs);

}
