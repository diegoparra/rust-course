#[derive(Clone, Copy)]
struct Dimension {
    width: f64,
    height: f64,
    depth: f64,
}

enum BoxColor {
    GREY,
    BROWN,
}

struct ShippingBox {
    weight: f64,
    dimension: Dimension,
    color: BoxColor,
}

impl Dimension {
    fn print_box_dimensions(&self) {
        println!("width: {}", self.width);
        println!("height: {}", self.height);
        println!("depth: {}", self.depth);
    }
}

impl BoxColor {
    fn print_box_color(&self) {
        match self {
            BoxColor::GREY => println!("Box Color: Grey"),
            BoxColor::BROWN => println!("Box Color: Brown"),
        }
    }
}
impl ShippingBox {
    fn create_box(weight: f64, dimension: Dimension, color: BoxColor) -> Self {
        Self {
            weight: weight,
            dimension: dimension,
            color: color,
        }
    }
    fn print_box_info(&self) {
        self.dimension.print_box_dimensions();
        self.color.print_box_color();
        println!("Box weight: {}", self.weight);
    }
}

fn main() {
    let small_box = Dimension {
        width: 10.0,
        height: 10.0,
        depth: 10.0,
    };

    let shipping_one = ShippingBox::create_box(20.0, small_box, BoxColor::GREY);
    shipping_one.print_box_info();

    let shipping_two = ShippingBox::create_box(20.0, small_box, BoxColor::GREY);
    shipping_two.print_box_info();
}
