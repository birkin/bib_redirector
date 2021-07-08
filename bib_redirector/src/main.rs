#[macro_use] extern crate rocket;

use bib_redirector::InfoHelper;

use rocket::response::Redirect;
use rocket::tokio::time::{sleep, Duration, Instant};


#[get("/")]
async fn index() -> &'static str {
    sleep(Duration::from_secs(2)).await;
    "coming: root-response"
}



#[get("/info")]
async fn info() -> &'static str {

    InfoHelper::print_elapsed().await;

    println!( "lib function call done" );

    "coming: info-response"

}


#[get("/misc")]
async fn misc() -> Redirect {

    let start = Instant::now();
    // let zz: () = start;  // yields: found struct `std::time::Instant`

    // sleep(Duration::from_secs(1)).await;
    let elapsed: Duration = start.elapsed();
    // let zz: () = elapsed;  // yields: found struct `Duration`

    println!( "elapsed time;, `{:?}`; about to redirect", elapsed );

    Redirect::to(uri!( "https://google.com" ))

    // "coming: misc-response"
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![info])
        .mount("/", routes![misc])
}
