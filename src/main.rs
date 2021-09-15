mod schema;

fn main (){
    println!("Compiles!");
    let test: schema::Authorize = Default::default();

    println!("{:?}", test);
}