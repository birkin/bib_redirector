Info
====

This [rust app](https://www.rust-lang.org) will...

- parse the bib out of the incoming url
- run an alma-api lookup to get the alma-id
- build a primo redirect url
- redirect the user to that url

yay!

---
---

Next
====

- implement regex for the validate-bib check, with tests.

- see if there's an easy way to implement a route with the same pattern, but bib_tester/info -- that'd get picked up before the bib-num pattern.

---
---


Misc
====

Since this'll be replaced, archiving here my first rust randomization code...

```
    pub async fn validate_bib( &self, bib: &str ) -> bool {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let i: u8 = rng.gen_range( 0..2 ); // low, 0; high, 1
        println!( "i, ``{:?}``", i );
        if i == 0 {
            false
        } else {
            true
        }
    }
```


---
---
