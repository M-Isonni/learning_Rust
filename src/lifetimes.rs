pub fn longest_string(){
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

}

fn longest<'a>(string1: &'a str, string2: &'a str)->&'a str{
    if string1.len()>string2.len(){
        string1
    }        
    else {
        string2
    }
}