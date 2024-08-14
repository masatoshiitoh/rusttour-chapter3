fn main() {
    let mut chaos = [3,5,4,1,2];
    chaos.sort();
    assert_eq!(chaos, [1,2,3,4,5]);
    println!("Hello, world! {:?}", chaos);

    let s = "hello".to_string().capacity();
    println!("hello's capacity = {}", s);

    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"));
    }

}
