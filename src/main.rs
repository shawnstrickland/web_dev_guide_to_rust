extern crate num_cpus;


fn convert_string_to_binary(string: &str) -> String {
    let name = string.to_string();
    let mut name_in_binary = "".to_string();

    // Call into_bytes() which returns a Vec<u8>, and iterate accordingly
    // I only called clone() because this for loop takes ownership
    for character in name.clone().into_bytes() {
        name_in_binary += &format!("0{:b} ", character);
    }

    return name_in_binary;
}

fn main() {
    println!("{}", num_cpus::get());
    let name = "Shawn";
    let name_in_binary = convert_string_to_binary(name);
    println!("\"{}\" in binary is {}", name, name_in_binary);
}