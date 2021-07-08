#[macro_use] extern crate rocket;


#[get("/")]
async fn index() -> &'static str {
    use rocket::tokio::time::{sleep, Duration};
    sleep(Duration::from_secs(2)).await;
    "coming: root-response"
}


#[get("/info")]
async fn info() -> &'static str {

    use bib_redirector::InfoHelper;

    InfoHelper::print_elapsed().await;

    println!( "lib function call done" );

    "coming: info-response"

}


#[get("/misc")]
async fn misc() -> &'static str {

    use rocket::tokio::time::{sleep, Duration, Instant};

    let start = Instant::now();
    // let zz: () = start;  // yields: found struct `std::time::Instant`

    // sleep(Duration::from_secs(1)).await;
    let elapsed: Duration = start.elapsed();
    // let zz: () = elapsed;  // yields: found struct `Duration`

    println!( "elapsed time, `{:?}`", elapsed );

    "coming: misc-response"
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![info])
        .mount("/", routes![misc])
}
