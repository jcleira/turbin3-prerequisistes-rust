# turbin3-prerequisites-rust
Welcome to my turbin3 prerequisites rust repository. This repository contains the rust prerequisites for my turbin3 cohort. 

I wanted to create this repository to write my thoughts and notes while working on the project, I hope you find them useful.

## Thoughts
### 1.1 Setting up
Pretty simple setup, I like it!

### 1.2 Generate a Keypair
It's pretty clear that the guide is a touch down as the key creation is mostly done to be stored in a file.

I'm publishing the generated publickey here, but I did a basic protection for the private key using `gitignore` (not super secure I know, but enought for this test).
```
CHnzs6t1E4mQdaGcP4FdXWRFDG9QBzu2QcTBnwk9FxRr
```

### 2. Claim Token Airdrop
Pretty straight forwardI've done this a few times so no problems on this part.

The output is pretty clear too:
```bash
turbin3-prerequisistes-rust git:(main) ✗ cargo test airdrop -- --nocapture
Compiling turbin3-prerequisistes-rust v0.1.0 (/Users/arvos/Code/rust/src/github.com/turbin3-prerequisistes-rust)
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.77s
Running unittests src/lib.rs (target/debug/deps/turbin3_prerequisistes_rust-be7a9123434a0575)

running 1 test
Success! Check out your TX here:
https://explorer.solana.com/tx/3jeERph2Xri5pDZ6koAaYDvZFJnfAvLfaJwrHPwnwdj4MkkNipKTA9XFHFgFAxQywQSPnxaPzTdPjfgaJHKcJTSZ?cluster=devnet
test tests::airdrop ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.55s
```

### 3. Transfer tokens to your WBA Address
Pretty straight forward too, basic transfer of tokens from one wallet to another.

```bash
turbin3-prerequisistes-rust git:(main) ✗ cargo test transfer_sol -- --nocapture
Compiling turbin3-prerequisistes-rust v0.1.0 (/Users/arvos/Code/rust/src/github.com/turbin3-prerequisistes-rust)
Finished `test` profile [unoptimized + debuginfo] target(s) in 3.01s
Running unittests src/lib.rs (target/debug/deps/turbin3_prerequisistes_rust-12985dc10796927d)

running 1 test
Success! Check out your TX here: https://explorer.solana.com/tx/Rm9USHF8wwW5YbmpeUGSAtPm6WnXt3E3LeAUHoTbWDtCE3xygZzbM5jz2qhiYKMvuKxTdk6zfiu4Vj8Xp6xhH6F/?cluster=devnet
test tests::transfer_sol ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 13.32s
```

### 4. Empty devnet wallet into WBA wallet
```bash
turbin3-prerequisistes-rust git:(main) ✗ cargo test transfer_all_sol -- --nocapture
Compiling turbin3-prerequisistes-rust v0.1.0 (/Users/arvos/Code/rust/src/github.com/turbin3-prerequisistes-rust)
Finished `test` profile [unoptimized + debuginfo] target(s) in 2.86s
Running unittests src/lib.rs (target/debug/deps/turbin3_prerequisistes_rust-12985dc10796927d)

running 1 test
Success! Check out your TX here: https://explorer.solana.com/tx/5Pk5eJPMYFxDWT3YeD9JC68Qh3tBLone1nLcXNWPeUBx5NQKX1Ecfm64NXGp7w44aWG9wzZotAUiV1Nc35CtFyqG/?cluster=devnet
test tests::transfer_all_sol ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 12.84s
```

### 5. Submit your completion of the WBA pre-requisites program
Again looks like this is the meat of the pre-requisites, I'm more familiar than before after doing it in TS.

Transaction is sent!, output:
```bash
turbin3-prerequisistes-rust git:(main) ✗ cargo test enroll -- --nocapture
Compiling turbin3-prerequisistes-rust v0.1.0 (/Users/arvos/Code/rust/src/github.com/turbin3-prerequisistes-rust)
Finished `test` profile [unoptimized + debuginfo] target(s) in 2.67s
Running unittests src/lib.rs (target/debug/deps/turbin3_prerequisistes_rust-4e53eac979ac96ef)

running 1 test
Success! Check out your TX here: https://explorer.solana.com/tx/5RjEeDwiwSRPAcZpT5BBw3vBVfEUwCHD5H3vGfNLYg52XxiDkxSzKZfwfCEpnzULMFDyQrMpn6D1wH4VMEmtLaGK/?cluster=devnet
test tests::enroll ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 13.33s
```

## Extras
Again, there would be a few things I would have done differently if this was a production code, but for the sake of the test, I think it's good enough.
