pub fn make_rectangle(){
    let width = 30;
    let height = 50;

    println!("the area of the rectangle is {}",area(width,height));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_tuple(dimensions:(u32,u32))->u32{
    dimensions.0*dimensions.1
}

pub fn make_rectangle_tuple(){
    let rect = (30,50);

    println!("the area of the rectangle with tuple is {}",area_with_tuple(rect));
}

#[derive(Debug)]//this helps println!() to print the values inside the structure passing just the instance of the struct to it.
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_with_struct(rect: &Rectangle)-> u32{
    rect.width*rect.height
}

pub fn make_rectangle_struct(){
    let rect= Rectangle{
        width:30,
        height: 50,
    };

    println!("the area of the rectangle with struct is {}",area_with_struct(&rect));

    println!("rect is: {:?}",rect);
}

//implementing methods called on istances of a struct

//implementing a function in a struct.
//first parameter of this kind of methods is always &self and it will be called
//by an istance of the struct
impl Rectangle{
    fn area(&self)->u32{
        self.width*self.height
    }
    fn can_hold(&self,other: &Rectangle)->bool{
        self.width >= other.width && self.height >= other.height
    }
}

pub fn make_rect_with_area_method(){
    let rect = Rectangle{
        width:30,
        height:50,
    };
    println!("rect area with implemented method is: {}",rect.area());
    let rect1=Rectangle{
        width:60,
        height:10,
    };

    let holding = rect.can_hold(&rect1);

    println!("{:?} can hold {:?}? {}",rect,rect1,holding);
}

//Associated functions:
//we can create functions without a reference to self, such functions will be called
//through the struct name (such as String::form(""))
//this kind of functions are usually used as constructors.
impl Rectangle{
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
    fn make(width: u32,height: u32) -> Rectangle {
        Rectangle { width: width, height: height }
    }
}

pub fn make_rect_constructor(){
     let square=Rectangle::square(10);
     println!("this is: {:?}",square);
     let rect= Rectangle::make(10,20);
     println!("this is: {:?}",rect);
}