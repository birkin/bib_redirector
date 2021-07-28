#[macro_use] extern crate rocket;

use bib_redirector::InfoHelper;
use bib_redirector::RedirectHelper;

// use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket::response::Redirect;
use rocket::tokio::time::{Duration, Instant};

// use rocket_dyn_templates::Template;

// use rocket::tokio::time::{sleep, Duration, Instant};


#[get("/")]
async fn root() -> &'static str {
    /*  Mini early example of just returning a response.
     */
    "coming: root-response redirect to /info"
}


#[get("/bib_redirect_tester/load_testing")]
async fn ld_tstng() -> String {
    /*  Mini early example of loading handler code from a different file (lib.rs).
     */

    // "coming: load-testing response"  // works with correct signature

    // let resp: String = "string_response".to_string();  // works with correct signature

    let resp: String = InfoHelper::return_elapsed().await;

    resp
}


#[get("/bib_redirect_tester/<bib>")]
async fn rdrctr(bib: String) -> Result< Redirect, &'static str > {
    /*  Happy-path will return a Redirect to the equivalent Primo bib-record (mms_id);
        Error will return for now, a message like "bad_bib".
        TODO: flow the error-string through a template, which may require changing the Error return-signature.
     */

    // -- setup
    println!( "perceived bibnum, ``{:?}``", bib);
    let redirector = RedirectHelper::new( &bib ).await;
    print!("alma-api-url, ``{:?}``", redirector.alma_api_url);

    // -- validate bib
    /*  see <https://github.com/SergioBenitez/Rocket/blob/693f4f9ee50057fc735e6e7037e6dee5b485ba10/examples/responders/src/main.rs#L79-L85>
        ...or `Either` <https://github.com/SergioBenitez/Rocket/blob/693f4f9ee50057fc735e6e7037e6dee5b485ba10/examples/responders/src/main.rs#L124-L140>
     */
    let is_valid: bool = redirector.validate_bib( &bib ).await;
    println!( "is_valid, ``{:?}``", is_valid );
    if is_valid == false {
        Err( "bad_bib" )  // reminder to future self: without the ending `;`, this is the return-value.
    } else {

        // -- add check-digit
        let updated_bib: String = redirector.add_check_digit( &bib ).await;
        println!( "updated_bibnum, ``{:?}``", updated_bib );

        // -- build api-url
        let url: String = redirector.build_api_url( &updated_bib ).await;
        println!( "api-url, ``{:?}``", url );

        // -- hit api
        let mms_id: String = redirector.hit_alma_api( &url ).await;
        println!( "mms_id, ``{:?}``", mms_id );

        // -- build redirect url
        let redirect_url_template: String = "https://brown.primo.exlibrisgroup.com/discovery/fulldisplay?docid=almaTHE_MMS_ID&context=L&vid=01BU_INST:BROWN".to_string();
        let redirect_url: String = str::replace( &redirect_url_template, "THE_MMS_ID", &mms_id );
        println!( "redirect_url, ``{:?}``", redirect_url );

        // -- happy path redirect
        // Ok( Redirect::moved(redirect_url) )
        Ok( Redirect::temporary(redirect_url) )  // `temporary`: prevents browser from caching the redirect

    } // end of is_valid

}


#[get("/info")]
async fn info() -> &'static str {
    /*  Mini early example of loading handler code from a different file (lib.rs).
     */
    InfoHelper::print_elapsed().await;
    println!( "lib function call done" );
    let resp: &str = "coming: info-response";
    resp
}


// #[get("/misc")]
// async fn misc() -> Template {
//     /*  Initial template test.
//      */
//     let start = Instant::now();
//     let elapsed: Duration = start.elapsed();
//     println!( "elapsed time;, `{:?}`; about to redirect", elapsed );

//     Template::render( "tera_test", context! {
//         elapsed: elapsed,
//         foo: "bar"
//     } )
// }

// #[get("/hello/<name>")]
// pub fn hello(name: &str) -> Template {
//     Template::render("tera/index", context! {
//         title: "Hello",
//         name: Some(name),
//         items: vec!["One", "Two", "Three"],
//     })
// }


#[get("/misc")]
async fn misc() -> &'static str {
    /*  Mini early example of coding before response.
     */
    let start = Instant::now();
    let elapsed: Duration = start.elapsed();
    println!( "elapsed time;, `{:?}`; about to redirect", elapsed );
    "coming: misc-response"
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![root])
        .mount("/", routes![ld_tstng])
        .mount("/", routes![rdrctr])
        .mount("/", routes![info])
        .mount("/", routes![misc])
        // .attach( Template::fairing() )
}
