// traits are like interfaces in java

trait Shape {
    fn get_area(&self) -> i32;
}

struct Square {
    side: i32,
}
#[derive(Debug)]
struct Rect {
    height: i32,
    width: i32,
}

impl Shape for Square {
    fn get_area(&self) -> i32 {
        self.side * self.side
    }
}

impl Shape for Rect {
    fn get_area(&self) -> i32 {
        self.height * self.width
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
           write!(f, "This is a square of side {} that implemented the display trait",self.side)
           }
}

pub fn traits() {
    let sq = Square { side: 10 };
    let rec = Rect {
        height: 20,
        width: 40,
    };

    let sq_area = sq.get_area();
    let rec_area = rec.get_area();

    println!(
        "Area of square is {} and area of rectange is {}",
        sq_area, rec_area
    );

    // debug and display traits
    println!("{}",sq); // will give error until unless we implement the display trait on square

    // We can also use the by default debug trait provided by rust we just have to do this #[derive(Debug)] for Rect
    println!("{:?}",rec);

    // Why there is no such macro for display then? Because display is smth that is seen by end users so rust doesnt knows how you wanna
    // show data to ur end users. Debug is just for developers that's why we can use it and there is a default implementation for it.
}


// NOTE
// Also before compilation all macros are expanded first then they are compiled
