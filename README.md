# particle-life-rust

A simulation of atoms with different interaction rules (attraction/repulsion) intended to demonstrate emergent, life-like behavior within the particles.


To use this app you'll need to build it yourself (at least until I figure out why Apple needs me to sign + notarize this app before its capable of running on other people's computers). Clone this repository and do the following:
1. install rust: https://www.rust-lang.org/tools/install
2. ensure you're using node v20
3. clone the repo
4. run `npm i` to install the necessary libraries
5. run `CI=true npm run tauri build` to build the app.

App should be built and available for use within `src-tauri/target/release/bundle/macos` (if you're on mac).

v1: 
https://github.com/whyismynamerudy/particle-life-rust/assets/64883022/5fbbd919-cc66-44ec-bd0a-73f3a21ff6b0

v2 (what is included in the release):
https://github.com/whyismynamerudy/particle-life-rust/assets/64883022/e46d370f-6c1d-4b7b-94d0-5cbba5e94fb7
