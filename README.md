Info
====

### This [rust](https://www.rust-lang.org) web-app is intended...

- to continue my exploration of rust
- to explore the [Rocket web-framework](https://rocket.rs)
- to explore coding async-first
- to explore rust/async testing

### Functionally, it...

- parses the bib out of the incoming url
- validates the bib
- runs an alma-api lookup to get the alma mms_id
- builds a primo redirect url
- redirects the user to that url

This all works now. (yay!)

### Notes...

- Our actual implementation-solution will likely be in php or python.

---


Next
====

- √ see if there's an easy way to implement a route with the same pattern, but bib_tester/info -- that'd get picked up before the bib-num pattern. A lightweight endpoint like this would allow experimenting with ab load-testing.

- look into returning a templated response.

- switch any simple unwrap() calls to unwrap_or_else() for my own edification -- to better understand the kinds of error-conditions that can exist.

- explore caching.

- explore saving api lookups into an sqlite db.

- look into how I might embed the curret git-commit-version into the compilation process, for my usual webapp version url-response.

---


Resources
=========

- [tutorial](https://dev.to/davidedelpapa/rocket-tutorial-01-basics-4ph9)

- [post on Tera](https://blog.logrocket.com/top-3-templating-libraries-for-rust/)
