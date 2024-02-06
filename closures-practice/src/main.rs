fn main() {
  let s = "🍓".to_string();

  let f = || {
    println!("strawberry  {}", s);
  };

    f();


    let v = vec![2,4,6];



    v.iter()
    .map(|x| x * 3)
    .filter(|x| x > &10)
    .fold(0, |acc, x| acc + x);

    println!("{:?}", v);



}
