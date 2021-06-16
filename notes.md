# mvp

- integrate logo into header

# todo

- redesign landing page
- (stretch) extract geo metadata from images
- make create page support multiple qrs
- change hrp's of treasure keys to match scheme of account keys
- add newtypes for treasure/account keys/signatures
- run rustfmt and clippy
- remove allow(unused) and clean up warnings
- clean up warnings in wasm build
- move signing into create_signature shared function
- change wasm/sign_with_secret_key to use map instead of a second fn
- add custom error type for route responses
- sign hashes, not full blobs
- remove unused info from response types
- implement treasure_page route, share treasure code with the recent_page route
- move create_treasure api to client side

# notes

- print stylesheets
  - https://www.matuzo.at/blog/i-totally-forgot-about-print-style-sheets/
- k256p native library

  > @brson nice blog post! I looked at signature verification on-chain last hackathon and put a summary of the instruction count needed here: https://gist.github.com/jgensler8/ded6fef052a028c4db4c112e80a4833c
  >
  > not sure if it 100% covers your use case but there is also a Secp256k1 Native Program (kinda like a standard library for cross program invocation) which might also help: https://docs.solana.com/developing/runtime-facilities/programs#secp256k1-program