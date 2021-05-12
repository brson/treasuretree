# todo

- add newtypes for treasure/account keys/signatures
- run rustfmt and clippy
- remove allow(unused) and clean up warnings
- clean up warnings in wasm build
- move signing into create_signature shared function
- change wasm/sign_with_secret_key to use map instead of a second fn
- move crypto_shared.rs to a crate
- add custom error type for route responses
- sign hashes, not full blobs
- remove unused info from response types
- implement treasure_page route, share treasure code with the recent_page route
- move create_treasure api to client side

# notes

- print stylesheets
  - https://www.matuzo.at/blog/i-totally-forgot-about-print-style-sheets/