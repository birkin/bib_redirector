// use std::collections::HashMap;
use std::env;
use std::env::VarError;


#[derive(Debug)]
pub struct RedirectHelper {
    pub perceived_bib: String,
    pub alma_api_url_template: String,
    pub alma_api_url: String,
    pub alma_redirect_url: String,
    api_key: String
}


    // "https://api-na.hosted.exlibrisgroup.com/almaws/v1/bibs?view=brief&expand=none&other_system_id=THE_BIB-01bu_inst&apikey=THE_API_KEY";


impl RedirectHelper {

    pub async fn new( bib: &str ) -> RedirectHelper {
        let perceived_bib = bib.to_string();
        let alma_api_url_template = "https://api-na.hosted.exlibrisgroup.com/almaws/v1/bibs?view=brief&expand=none&other_system_id=THE_BIB-01bu_inst&apikey=THE_API_KEY".to_string();
        let alma_api_url = "".to_string();
        let alma_redirect_url = "".to_string();
        let api_key: Result<String, VarError> = env::var("BIB_REDIRECT_TEST__ALMA_API_KEY");
        match api_key {
            Ok(_) => {},
            Err(_err) => {
                println!("api-key envar not found; quitting");
                std::process::exit(-1);
            }
        };
        let api_key: String = api_key.unwrap();  // this is ok because any error is handled above
        RedirectHelper { perceived_bib, alma_api_url_template, alma_api_url, alma_redirect_url, api_key }
    }


    // pub async fn new( bib: &str ) -> RedirectHelper {

    //     // -- incorporate bib into url-template
    //     println!("bib in helper, ``{:?}``", bib);
    //     let alma_api_url_template: String = String::from("https://api-na.hosted.exlibrisgroup.com/almaws/v1/bibs?view=brief&expand=none&other_system_id=THE_BIB-01bu_inst&apikey=THE_API_KEY");
    //     println!("alma_api_url_template, ``{:?}``", alma_api_url_template);
    //     let url_with_bib: String = str::replace( &alma_api_url_template, "THE_BIB", bib );
    //     println!("url_with_bib, ``{:?}``", url_with_bib);

    //     // -- incorporate api-key into url-template
    //     // let api_key = env::var( "BIB_REDIRECT_TEST__ALMA_API_KEY" );  // returns Result
    //     let api_key: Result<String, VarError> = env::var("BIB_REDIRECT_TEST__ALMA_API_KEY");
    //     match api_key {
    //         Ok(_) => {},
    //         Err(_err) => {
    //             println!("api-key envar not found; quitting");
    //             std::process::exit(-1);
    //         }
    //     };
    //     let api_key: String = api_key.unwrap();  // this is ok because the error is handled above
    //     println!("api_key, ``{:?}``", api_key);
    //     let url_with_key: String = str::replace( &url_with_bib, "THE_API_KEY", &api_key );
    //     let alma_api_url: String = url_with_key;
    //     let alma_redirect_url: String = "".to_string();

    //     RedirectHelper { alma_api_url, alma_redirect_url }
    // }

    pub async fn add_check_digit( &self, bib: &str ) -> String {
        println!( "initial bib, ``{:?}``", bib );
        "foo".to_string()
    }


    pub async fn hit_alma_api( &self ) -> Result< (), Box<dyn std::error::Error> > {

        let client = reqwest::Client::builder().build()?;  // <https://dev.to/pintuch/rust-reqwest-examples-10ff>


        let res = client
            .get("https://httpbin.org/ip")
            .send()
            .await?;
        // let ip = res
        //     .json::<HashMap<String, String>>()
        //     .await?;
        // println!( "ip, ``{:?}``", ip );

        let res_txt: String = res.text().await?;
        println!( "res_txt, ``{:?}``", res_txt);


        let resp = client
            .get( &self.alma_api_url )
            .header( "accept", "application/json" )
            .send()
            .await?;
            // .json::<HashMap<String, String>>()
            // .await?;

        // println!("resp, ``{:#?}``", resp);
        // println!( "resp.text(), ``{:#?}``", resp.text().await? );

        let rsp_txt: String = resp.text().await?;
        // let zz: () = rsp_txt;
        println!( "rsp_txt, ``{:?}``", rsp_txt);

        // println!( "resp.json, ``{:#?}``", resp.json::<HashMap<String, String>>().await? );  // doesn't work


        Ok(())

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
