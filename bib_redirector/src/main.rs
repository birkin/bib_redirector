#[macro_use] extern crate rocket;

use bib_redirector::InfoHelper;
use bib_redirector::RedirectHelper;

use rocket::response::Redirect;
use rocket::tokio::time::{Duration, Instant};
// use rocket::{Request};

// use rocket::tokio::time::{sleep, Duration, Instant};


// #[get("/")]
// async fn index() -> Redirect {

//     Redirect::moved(uri!( "https://www.google.com/" ))

//     // sleep(Duration::from_secs(2)).await;
//     // "coming: root-response"
// }


#[get("/")]
async fn root() -> &'static str {

    // Redirect::moved(uri!( "https://www.google.com/" ))

    // sleep(Duration::from_secs(2)).await;
    "coming: root-response redirect to /info"
}


#[get("/bib_redirect_tester/<bib>")]
async fn tester(bib: String) -> Redirect {
    println!( "the bibnum, ``{:?}``", bib);
    let redirector = RedirectHelper::new( &bib ).await;
    // Redirect::moved(uri!( "https://www.google.com/" ))
    Redirect::temporary(uri!( "https://www.google.com/" )) // useful for testing, so browser doesn't cache it

}


#[get("/info")]
async fn info() -> &'static str {

    InfoHelper::print_elapsed().await;

    println!( "lib function call done" );

    "coming: info-response"

}


#[get("/misc")]
async fn misc() -> &'static str {

    let start = Instant::now();
    // let zz: () = start;  // yields: found struct `std::time::Instant`

    // sleep(Duration::from_secs(1)).await;
    let elapsed: Duration = start.elapsed();
    // let zz: () = elapsed;  // yields: found struct `Duration`

    println!( "elapsed time;, `{:?}`; about to redirect", elapsed );

    // Redirect::to(uri!( "https://google.com" ))

    "coming: misc-response"
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![root])
        .mount("/", routes![tester])
        .mount("/", routes![info])
        .mount("/", routes![misc])
}
