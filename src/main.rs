fn main() {
    println!("Hello, world!");
    // let e:i32 = 100;
    let is_snowing: bool = true;
    println!("is snowing? {}", is_snowing);
    // let letter: char = 'a';
    let letterstring: &str = "aaaaaa";
    println!("letter string is: {}{}", letterstring, "ddfddf");
    let numbers : [i32; 5] = [1, 2, 3, 4, 5];
    println!("the numbers are: {:?}", numbers);

    // Arrays
    let fruits: [&str; 3] = ["apple", "banana", "orange"];
    println!("the fruits are: {:?}", fruits);

    let fruits: Vec<&str> = vec!["apple", "banana", "orange"];
    println!("the fruits are: {:?}", fruits);

    // Tuples
    let person: (String, i32, bool) = ("Alice".to_string(), 30, true); 
    println!("the person is: {:?}", person);
    let my_mix_tuple = ("Kratos", 30, true, [1,2,3,4,5]);
    println!("the mix tuple is: {:?}", my_mix_tuple);

    // Slices contagiously in memory
    let number_slices: &[i32; 5] = &[1,2,3,4,5];
    println!("the number slices are: {:?}", number_slices);
}