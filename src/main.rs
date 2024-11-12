fn main() {
    // Average
    let x : f64 = 5.0;
    let y : f64 = 10.0;
    let z : f64 = average(x, y);
    println!("The average of {} and {} is {}", x, y, z);
    println!(); // saut de ligne

    // 2. Rectangle
    let my_rectangle = Rectangle {
        length: 5.0,
        width: 10.0,
    };

    // let my_perimeter = perimeter(my_rectangle); /!\ Error: use of moved value: `my_rectangle`
    // Solution 1 : Créer une fonction perimeter2() qui prend un pointeur vers le rectangle et renvoie son paramètre
    let my_perimeter = perimeter2(&my_rectangle);
    println!("The perimeter of the rectangle is {}", my_perimeter);
    let my_perimeter = perimeter2(&my_rectangle);
    println!("The perimeter of the rectangle is {}. This now works twice !", my_perimeter);
    println!(); // saut de ligne

    // Solution 2 : Cloner la valeur du rectangle (voir modif dans struct #[derive(Clone)])
    let rectangle2 = my_rectangle.clone();
    let my_perimeter = perimeter(my_rectangle);
    let my_perimeter_clone = perimeter(rectangle2);
    println!("The perimeter of the rectangle is {}, and the one of the clone is {}", my_perimeter, my_perimeter_clone);
    println!(); // saut de ligne

    // Solution 3 : Utiliser la copie (voir modif dans struct #[derive(Copy)])
    let my_perimetre1 = perimeter(my_rectangle);
    let my_perimetre2 = perimeter(my_rectangle);
    println!("The perimeter of the rectangle is {}, and the one of the clone is {}. This now works twice !", my_perimetre1, my_perimetre2);
    println!(); // saut de ligne
}

// 1. Average
fn average(float1 : f64, float2 : f64) -> f64 {
    (float1 + float2) / 2.0
}

// 2. Rectanlge . . .

#[derive(Clone, Copy)]
struct Rectangle {
    length: f64,
    width: f64,
}

fn perimeter(rectangle: Rectangle) -> f64 {
    2.0 * (rectangle.length + rectangle.width)
}

fn perimeter2(rectangle: &Rectangle) -> f64 {
    2.0 * (rectangle.length + rectangle.width)
}