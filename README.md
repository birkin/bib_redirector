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

next:
- try the simple-test again trying to load the json into an unknwown structure, like:

    `let value: serde_json::Value = serde_json::from_str(j).unwrap();`

    ...as seen here: <https://github.com/serde-rs/json/issues/144>

    - also interesting: <https://stackoverflow.com/questions/58233949/how-can-i-use-serde-json-on-a-json-object-with-variable-key-names>

    - also see: <https://blog.logrocket.com/json-and-rust-why-serde_json-is-the-top-choice/>

        - specifically the section "Working without types"

---
---
