use std::env;
use std::env::VarError;


#[derive(Debug)]
pub struct RedirectHelper {
    alma_api_url_template: String,

}


    // "https://api-na.hosted.exlibrisgroup.com/almaws/v1/bibs?view=brief&expand=none&other_system_id=THE_BIB-01bu_inst&apikey=THE_API_KEY";


impl RedirectHelper {

    pub async fn new( bib: &str ) -> RedirectHelper {
        // -- incorporate bib into url-template
        println!("bib in helper, ``{:?}``", bib);
        let alma_api_url_template = String::from("https://api-na.hosted.exlibrisgroup.com/almaws/v1/bibs?view=brief&expand=none&other_system_id=THE_BIB-01bu_inst&apikey=THE_API_KEY");
        println!("alma_api_url_template, ``{:?}``", alma_api_url_template);
        let url_with_bib = str::replace( &alma_api_url_template, "THE_BIB", bib );
        // let zz: () = url_with_bib;  // yields, found struct `std::string::String`
        println!("url_with_bib, ``{:?}``", url_with_bib);

        // -- incorporate api-key into url-template
        // let api_key = env::var( "BIB_REDIRECT_TEST__ALMA_API_KEY" );  // returns Result
        let api_key: Result<String, VarError> = env::var("BIB_REDIRECT_TEST__ALMA_API_KEY");
        match api_key {
            Ok(_) => {},
            Err(_err) => {
                println!("api-key envar not found; quitting");
                std::process::exit(-1);
            }
        };
        let api_key: String = api_key.unwrap();  // this is ok because the error is handled above
        println!("api_key, ``{:?}``", api_key);
        // let zz: () = api_key;

        RedirectHelper { alma_api_url_template }
    }

}



#[derive(Debug)]
pub struct InfoHelper {
    content: String,
}

impl InfoHelper {
    pub async fn print_elapsed() {
        use rocket::tokio::time::{sleep, Duration, Instant};

        let start = Instant::now();
        // let zz: () = start;  // yields: found struct `std::time::Instant`

        sleep(Duration::from_secs(1)).await;        // original line 2

        let elapsed: Duration = start.elapsed();
        // let zz: () = elapsed;  // yields: found struct `Duration`

        println!( "elapsed time, `{:?}`", elapsed );
    }
}
