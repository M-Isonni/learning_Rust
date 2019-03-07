pub fn make_loop(){
    let mut counter = 0;
    let result = loop{
        counter+=1;
        println!("counter = {}",counter);
        if counter==20{                
                break counter *2;
        }
    };
    assert_eq!(result, 40);
    println!("result {}", result);
}

pub fn make_while(){
    let mut counter = 0;    
    while counter < 10{
        counter +=1;
        println! ("number = {}",counter);
    };

    println!("left while");
}

pub fn make_for(){
    let array=[10,15,20,25,30,35];
    for element in array.iter(){
        println!("element is {}", element);
    }
}

pub fn make_rev_for(){
    for number in (1..4).rev() {
        println!("number = {}", number);
    }
}

pub fn make_range_for(){
    for number in 1..4 {
        println!("number = {}", number);
    }
}