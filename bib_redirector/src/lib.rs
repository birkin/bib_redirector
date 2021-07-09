#[derive(Debug)]
pub struct RedirectHelper {
    alma_api_url_template: String,

}


    // "https://api-na.hosted.exlibrisgroup.com/almaws/v1/bibs?view=brief&expand=none&other_system_id=THE_BIB-01bu_inst&apikey=THE_API_KEY";


impl RedirectHelper {

    pub async fn new( bib: &str ) -> RedirectHelper {
        println!("bib in helper, ``{:?}``", bib);
        let alma_api_url_template = String::from("https://api-na.hosted.exlibrisgroup.com/almaws/v1/bibs?view=brief&expand=none&other_system_id=THE_BIB-01bu_inst&apikey=THE_API_KEY");
        println!("alma_api_url_template, ``{:?}``", alma_api_url_template);
        let url_with_bib = str::replace( &alma_api_url_template, "THE_BIB", bib );
        // let zz: () = url_with_bib;  // yields, found struct `std::string::String`
        println!("url_with_bib, ``{:?}``", url_with_bib);

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
