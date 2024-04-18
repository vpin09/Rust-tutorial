fn print_country(country_name: String) -> String {
    println!("{}", country_name);
    country_name
} 

fn main() {
    let country = String::from("Austria");
   let country=print_country(country); // We print "Austria"
    print_country(country); // ⚠️ That was fun, let's do it again!
}