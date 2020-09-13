use njord::geometry::vector::Vector;

fn main() {
    let v1 = Vector {
        dx: 10., 
        dy:10., 
        dz: 10. 
    };

    let v2 = Vector {
        dx: 30., 
        dy: 22., 
        dz: 35.5 
    };

    println!("Distance between {:?} and {:?}", v1, v2);
    //println!("is {}", p1.dist(&p2));
    let v3 = v1 + v2;
    println!("Vector sum is {:?}", v3);
}
