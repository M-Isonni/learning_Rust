
pub fn extracting_function(){
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for &number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let second_largest = extracted_fn(&number_list);
    println!("largest number from extracted fn is: {}", second_largest);
}

fn extracted_fn(i:&[i32])->i32{
    let mut largest:i32 = i[0];
    for &number in i.iter(){
        if largest<number{
            largest=number;
        }
    }
    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn general_larget<T: PartialOrd + Clone>(list: &[T])->T{
    let mut largest = list[0].clone();

    for item in list.iter(){
        if largest<*item{
            largest=item.clone();
        }
    }
    largest
}

pub fn make_largest(){
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

}

struct Point<T, U> {
    x: T,
    y: U,
}

//implementation of method only on the point structure made from 2 floats
//so only the type Point<f32,f32> will have this method, any other types of Point<T,U> won't
impl Point<f32,f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl<T: Clone, U: Clone> Point<T, U> {
    fn clone(&self)-> Point<T,U>{         
        Point{            
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}


pub fn make_generic_point() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let both_float_second = Point { x: 3.0, y: 6.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };   
    let p3=both_float.clone(); 

    let result_point = both_float.mixup(both_float_second);
    
    let distance = p3.distance_from_origin();

    println!("distance from origin of p1 is: {}", distance);
    println!("p3.x = {}, p3.y = {}",result_point.x,result_point.y);
}

