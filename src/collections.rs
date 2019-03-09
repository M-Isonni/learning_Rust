use std::collections::HashMap;

pub fn make_vector(){
    let mut vector:Vec<i32> = Vec::new();
    let vector2=vec![1,2,3,4,5];

    vector.push(8);

    println!("value: {}",&vector[0]);
    println!("value from vector2 : {}", vector2[0]);
}

pub fn get_vector_elem(){
    
    let v = vec![1, 2, 3, 4, 5];

    //we can get the element in a position of the vector through indexing
    let third: &i32 = &v[2];
    let second = v[1];
    println!("The third element is {}", third);
    println!("The second element is {}", second);

    //or through matching: matching is safest cause it manages the none case.s
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

pub fn vector_already_borrowed(){
    
    let mut v = vec![1, 2, 3, 4, 5];

    let first = v[0];

    v.push(6);

    println!("The first element is: {}", first);

    //cannot execute the code cause vector is first borrowed by &v[0] as immutable and then from v.push as mutable.
    //as we said only more immutable borrowing can exist at a time.
}

pub fn iterate_and_mutate(){
    let mut v = vec![2,3,4,5,6];
    for i in &mut v{
        println!("value= {}",i);
    }

    for i in &mut v{
        *i+=50;
        println!("new value is {}",i);
    }
}

pub fn make_hashmap(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Yellow"), 10);
    scores.insert(String::from("Blue"), 50);

    println!("yellow score: {}", scores["Yellow"]);
}

pub fn make_hashmap_from_tuples(){
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    let team_name = String::from("Blue");    
    
    let score_opt = scores.get(&team_name);

    let score = hashmap_value(&score_opt);

    println!("blue score: {}", score);
}

fn hashmap_value(hash: &Option<&&i32>) -> i32{   
    match *hash{
        Some(value)=>**value,
        None=>0,
    }
}

pub fn make_hasmap_owning_items(){
    //for types that have copy trait they are copied inside the hashmap
    //owned values like string instead will be owned by the hashmap, hence they will no longer be
    //usable after being put in the hashmap from the old variables.
    let key = String::from("color");
    let value = String::from("blue"); 

    let mut hash = HashMap::new();

    //inserting the references to the values instead of the values itself in the hashmap,
    //the hashmap won't own the values so they will be reusable after inserting them.
    hash.insert(&key, &value);
   
   //we can access a value of the hashmap through indexing
    println!("value is: {}",hash[&key]);
    

    //or through get
    let color:Option<&&String> = hash.get(&key);
    
    //however if we get the value it will return a Option<T> type
    //so we will need to manage and match the possibility of None
    println!("value: {}", hashmap_value_string(color));   
}

//function to manage the none possibility of the get from an hashmap <&string,&string>
fn hashmap_value_string(hash: Option<&&String>) -> String{   
    match hash{
        Some(value)=>String::from(value.as_str()),
        None=>String::from(" "),
    }
}

//iterating through an hashmap
pub fn iterating_hashmap(){
    let mut hash = HashMap::new();
    hash.insert(0,1);
    hash.insert(1,2);

    for(key,value) in &hash{
        println!("key: {}, value: {}",key,value);
    }
}

//managing keys with a value already assigned
pub fn hashmap_managing_keys(){
    overwrite_values();
    ignore_new_value();
    update_value();
}

//overwriting existing values
fn overwrite_values(){
    let mut scores = HashMap::new();
    //by default insert method if finds an already existing key will overwrite its value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}

//here we insert a new value only if the key doesn't already have a value
fn ignore_new_value(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    //or_insert returns a mutable reference to the value related to the key given with entry, if the key exist, otherwise if the
    // key doesn't exist or has a None value, it will insert the value
    //between ()
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

//updating value based on old value
fn update_value(){

    //considering or_insert returns a mutable reference to the value of a key, if the key exists, we can use it to 
    //modify the old values
    let mut hash = HashMap::new();
    let text=String::from("hello world wonderful world");

    for word in text.split_whitespace(){
        let count = hash.entry(word).or_insert(0);
        *count+=1;
    }

    for(key,value) in &hash{
        println!("key: {}, value: {}",key,value);
    }
}
