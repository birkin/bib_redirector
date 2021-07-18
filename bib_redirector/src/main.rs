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



// enum TypeOr<S, T> {
//     Left(S),
//     Right(T),
// }


// enum StrOrStrAndVec<'a> {
//     Str(&'a str),
//     StrAndVec(&'a str, Vec<usize>),
// }

// fn f3(flag: bool) -> StrOrStrAndVec<'static> {
//     if flag {
//         StrOrStrAndVec::StrAndVec("abc", vec![0, 1, 2])
//     } else {
//         StrOrStrAndVec::Str("abc")
//     }
// }


#[get("/bib_redirect_tester/<bib>")]
// async fn rdrctr(bib: String) -> Redirect {
// async fn rdrctr(bib: String) -> ResponseOrRedirect<'static> {
// async fn rdrctr(bib: String) -> Result<&'static str, Redirect> {
async fn rdrctr(bib: String) -> Result< Redirect, &'static str > {

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
        Err( "bad_bib" )
    } else {
        Ok( Redirect::temporary(uri!( "https://www.google.com/")) )
    }


    // #[get("/redir/<name>")]
    // fn maybe_redir(name: &str) -> Result<&'static str, Redirect> {
    //     match name {
    //         "Sergio" => Ok("Hello, Sergio!"),
    //         _ => Err(Redirect::to(uri!(redir_login))),
    //     }
    // }


    // -- add check-digit
    // let updated_bib: String = redirector.add_check_digit( &bib ).await;
    // println!( "updated_bibnum, ``{:?}``", updated_bib );

    // // -- build api-url
    // let url: String = redirector.build_api_url( &updated_bib ).await;
    // println!( "api-url, ``{:?}``", url );

    // // -- hit api
    // let mms_id: String = redirector.hit_alma_api( &url ).await;
    // println!( "mms_id, ``{:?}``", mms_id );

    // // -- build redirect url
    // let redirect_url_template: String = "https://brown.primo.exlibrisgroup.com/discovery/fulldisplay?docid=almaTHE_MMS_ID&context=L&vid=01BU_INST:BROWN".to_string();
    // let redirect_url: String = str::replace( &redirect_url_template, "THE_MMS_ID", &mms_id );
    // println!( "redirect_url, ``{:?}``", redirect_url );

    // -- happy path redirect
    // Redirect::moved( uri!( "https://www.google.com/" ) )  // works
    // Redirect::temporary( uri!( "https://www.google.com/" ) )  // works

    // Ok( Redirect::temporary( redirect_url ) )
    // ResponseOrRedirect::Rdrct(Redirect::temporary( redirect_url ))

    // Redirect::temporary( redirect_url )

}


#[get("/info")]
async fn info() -> &'static str {

    InfoHelper::print_elapsed().await;

    println!( "lib function call done" );

    let resp: &str = "coming: info-response";

    resp

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
        .mount("/", routes![rdrctr])
        .mount("/", routes![info])
        .mount("/", routes![misc])
}
