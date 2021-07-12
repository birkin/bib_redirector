// use std::collections::HashMap;
use std::env;
use std::env::VarError;


#[derive(Debug)]
pub struct RedirectHelper {
    pub perceived_bib: String,
    pub alma_api_url_template: String,
    pub alma_api_url: String,
    pub alma_item_url: String,
    api_key: String
}


impl RedirectHelper {

    pub async fn new( bib: &str ) -> RedirectHelper {
        let perceived_bib = bib.to_string();
        let alma_api_url_template = "https://api-na.hosted.exlibrisgroup.com/almaws/v1/bibs?view=brief&expand=none&other_system_id=THE_BIB-01bu_inst&apikey=THE_API_KEY".to_string();
        let alma_api_url = "".to_string();
        let alma_item_url = "".to_string();
        let api_key: Result<String, VarError> = env::var("BIB_REDIRECT_TEST__ALMA_API_KEY");
        match api_key {
            Ok(_) => {},
            Err(_err) => {
                println!("api-key envar not found; quitting");
                std::process::exit(-1);
            }
        };
        let api_key: String = api_key.unwrap();  // this is ok because any error is handled above
        RedirectHelper { perceived_bib, alma_api_url_template, alma_api_url, alma_item_url, api_key }
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
        println!( "incoming bib, ``{:?}``", bib );
        let initial_bib: String = bib.to_string();
        println!( "initial_bib, ``{:?}``", initial_bib );
        "fooz".to_string()
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


#[cfg(test)]
mod tests {
    use super::*;  // gives access to RedirectHelper struct

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[rocket::async_test]  // figured this out from <https://blog.x5ff.xyz/blog/async-tests-tokio-rust/>, and then looking at <https://github.com/SergioBenitez/Rocket/blob/677790d6397147f83066a284ee962bc174c555b5/examples/testing/src/async_required.rs#L25>
    async fn test_redirector_new_for_stored_bib() {
        let redirector = RedirectHelper::new( "b1234567" ).await;
        assert_eq!( "b1234567".to_string(), redirector.perceived_bib );
    }

    #[rocket::async_test]
    async fn test_add_check_digit_regular() {
        let redirector = RedirectHelper::new( "b1049798" ).await;
        let updated_bib: String = redirector.add_check_digit(&redirector.perceived_bib).await;
        assert_eq!( "b10497985".to_string(), updated_bib );
    }

    #[rocket::async_test]
    async fn test_add_check_digit_x() {
        let redirector = RedirectHelper::new( "b1102947" ).await;
        let updated_bib: String = redirector.add_check_digit(&redirector.perceived_bib).await;
        assert_eq!( "b1102947x".to_string(), updated_bib );
    }

}
