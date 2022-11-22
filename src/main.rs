
// mod first_contact;
// mod stringy_stuff;

// async fn call_greet() {
//     first_contact::greet_fast_alien().await;

// }

fn find_needle(){
    let needle = 0o204; // 132
    let haystack = [1,1,2,5,15,52,203,877,4140,132, 21147];

    for item in &haystack{
        if *item !=  needle {
            continue;
        }
        else{
            println!("Found it, it was: {}", item);
            break
        }
    }
}
fn main() {
    
    find_needle();

    // stringy_stuff::stringy();
    println!("")
}
