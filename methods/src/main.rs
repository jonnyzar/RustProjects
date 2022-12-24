fn main() {

    let rect1 = Rectangle {
        height: 30,
        width: 30,
    };

    println!(
        "Area is {}\nIs it a kvadrat? {}",
        rect1.area(), rect1.check_quad()
    );


    let ferrari = Car{
        position: (0,0,0),
        color: (255,0,0),
    };

    println!(
        "This is a {} Ferrari\nStanding at {:?}
And it is a Kvadrat? {}",
        ferrari.color_red(), ferrari.position, ferrari.form(&rect1)
    );

    let big = Car::truck(10);

    println!("Position of Truck is {:?}", big.position);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn check_quad(&self) -> bool {
        self.width == self.height
    }
}

struct Car{
    position: (i32,i32,i32),
    color: (i32,i32,i32),
}

impl Car{
    fn color_red (&self) -> bool {
        self.color.0 == 255 
    }

    //other1 can be any other name and reference
    fn form (&self, other1: &Rectangle) -> bool {
        other1.check_quad() == true
    }

    fn truck (size: i32) -> Self {
        Self{
            position:(size,10 * size,size),
            color: (250,0,0),
        }
    }
}