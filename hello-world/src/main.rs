fn main() {
    greet_world();
}

fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let brazil = "Olá, mundo!";
    let regions = [southern_germany, japan, brazil];
    for region in regions.iter() {
        println!("{}", &region);
    }
}
