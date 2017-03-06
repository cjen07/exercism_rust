pub fn raindrops(num: u32) -> String {
    let mut result: String = "".to_string();
    if num % 3 == 0 {
        result = result + "Pling";
    } 
    if num % 5 == 0 {
        result = result + "Plang";
    } 
    if num % 7 == 0 {
        result = result + "Plong";
    }

    if result == "" {
        result = num.to_string();
    }
    result
}