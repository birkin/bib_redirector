// #[macro_use] extern crate rocket;


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
