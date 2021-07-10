#[macro_use] extern crate rocket;

use bib_redirector::InfoHelper;
use bib_redirector::RedirectHelper;

use rocket::response::Redirect;
use rocket::tokio::time::{Duration, Instant};

// use rocket::tokio::time::{sleep, Duration, Instant};


#[get("/")]
async fn root() -> &'static str {
    "coming: root-response redirect to /info"
}


#[get("/bib_redirect_tester/<bib>")]
async fn tester(bib: String) -> Redirect {

    // -- setup
    println!( "the bibnum, ``{:?}``", bib);
    let redirector = RedirectHelper::new( &bib ).await;  // creates `alma_api_url`
    print!("alma-api-url, ``{:?}``", redirector.alma_api_url);

    // -- hit api
    let data = redirector.hit_alma_api().await;
    match data {
        Ok(_) => {},
        Err(_err) => {
            // println!("problem hitting alma-api; quitting");
            println!( "problem hitting alma-api, ``{:?}``; quitting", _err);
            std::process::exit(-1);
        }
    };


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
    let elapsed: Duration = start.elapsed();

    println!( "elapsed time;, `{:?}`; about to redirect", elapsed );

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
