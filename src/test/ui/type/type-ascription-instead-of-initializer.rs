fn main() {
    let x: Vec::with_capacity(10, 20);  //~ ERROR expected type, found `10`
    //~^ ERROR arguments to this function are incorrect
}
