# HARDBACK [![][img_license]](#license) [![][img_loc]][loc]

[img_license]: https://img.shields.io/badge/License-MIT_or_Apache_2.0-blue.svg
[img_loc]: https://tokei.rs/b1/github/alanpoon/hardback_conrod

Hardback is a turn-based word forming card game written in [Rust]. First player to reach 60victory points, wins! You can earn coins to buy better Letter Cards. If you have good vocabulary, you can form longer words to get the Literacy award which worth alot of victory points. If you don't, you can still stand a chance to win by forming matching genre cards to earn more victory point. Collect Inks to draw more cards into your hand to form longer words. Collect and make use of Timeless Classic cards that won't be shuffled back to the discard pile whereever you can.

[Rust]: https://www.rust-lang.org

![](https://i.imgur.com/uOwrmIV.png)

**Actual Website for Hardback**:
https://www.kickstarter.com/projects/fowers/hardback-the-pre-quill-to-paperback

**News**: [@rustropy gaming on twitter](https://twitter.com/rustropy_gaming) |
[devlog on imgur](https://imgur.com/a/SMVqO)

**Status**:
[![][img_travis-ci]][travis-ci]
[![][img_appveyor-ci]][appveyor-ci]
[![][img_circle-ci]][circle-ci]

[img_travis-ci]: https://img.shields.io/travis/alanpoon/hardback_conrod/master.svg?label=Linux|OSX
[img_appveyor-ci]: https://img.shields.io/appveyor/ci/alanpoon/hardback_conrod.svg?label=Windows
[img_circle-ci]: https://img.shields.io/circleci/project/github/alanpoon/hardback_conrod/master.svg?label=Android

[loc]: https://github.com/Aaronepower/tokei
[travis-ci]: https://travis-ci.org/alanpoon/hardback_conrod
[appveyor-ci]: https://ci.appveyor.com/project/alanpoon/hardback_conrod
[circle-ci]: https://circleci.com/gh/alanpoon/hardback_conrod


## Precompiled Binaries

Precompiled binaries for Linux, Windows, OS X and Android:
<https://github.com/alanpoon/hardback_conrod/releases>


## Screenshots

![](https://i.imgur.com/myVVfUW.png)

![](https://i.imgur.com/oXpIvb9.png)


## Gifs

![](https://i.imgur.com/ovrTxqy.gif)

![](https://i.imgur.com/VJaXQEJ.gif)


## Building from Source

```bash
# Clone this repo
git clone https://github.com/alanpoon/hardback_conrod
cd hardback_conrod

# Assets are stored in a separate repo.
# hardback_conrod expects them to be in `assets` directory.
git clone https://github.com/alanpoon/hardback_conrod_assets assets

# Compile a debug version
cargo build

# Run it
cargo run
```


## Building from Source for Android

[Set up a build environment][android setup] and run `./do_android` script:

![android screenshot](https://i.imgur.com/T9EgPR1.png)

[android setup]: https://github.com/tomaka/android-rs-glue#setting-up-your-environment


## License

hardback_conrod is distributed under the terms of both
the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE] and [LICENSE-MIT] for details.

[LICENSE-MIT]: LICENSE-MIT
[LICENSE-APACHE]: LICENSE-APACHE
