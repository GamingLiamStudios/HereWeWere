# Here We Were

Description under construction.

## What is this?

This is the complete source-code of my in-development game `Here We Were`, a JRPG-like Mystery Adventure game with a completely dynamic storyline.

## How do I play this?

By compiling it, of course!

### Compiling

To compile this, you will need the correct development environment for Bevy, the engine of choice for this project. You can find the instructions for that [here](https://bevyengine.org/learn/book/getting-started/setup/).

You will also need `mold` (Linux) or `lld` (Windows, MacOS). To install this on Windows, run these commands:

```powershell
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```

Then, to compile, simply run `cargo build --release --no-default-features` in the root directory of the project. This will compile the game into the `target/release` directory.

## So what is with the License?

The Engine (everything that isn't in `SOURCE/resources`) is fully OSS under `LGPL-2.1-only`. The Game Assets however, are not and **All Rights are Reserved**.
So you CAN use the Engine for your own projects, have them paid, etc. However ANY modification to the Engine MUST be released publicly under the same LGPL-2.1-only license.
You CANNOT use (distribute) the Assets, paid or not, without our \[GLStudios] explicit permission, in any commercial capacity. Memes are fine, but Games are not.

## Why make this Open Source?

I believe that anyone should be able to play my game. I would of course rather people buy it, but I am very aware that piracy is what others would prefer. I myself am guilty of this, even for games I really enjoyed.
By making this OSS, It allows those people - who cannot or choose not to buy the game - to still play it without any risk of Malware or any other of those nasty things.
There will also be a compile-time script that will scrape the commit pool and insert all contributors into the credits.
This also has the benefit of allowing fixes, graphical upgrades, etc. to be made by the community, which I can then merge into the main branch and release everywhere else, allowing for a better experience for everyone.
For these community resource contributions, any contributions will be considered a donation to the project, and therefor will be considered as property of GLStudios.

For profits from Binary Distribution (Steam, Itch, EG, etc.), I am considering a profit sharing system as follows:

- 60% of profits will go to the GLStudios team.
- 40% of profits will be split across Asset contributors per file.

Note that this is entirely just something I thought of in the moment and is very much subject to future discussion and change, as this is a very complex issue that I have no idea how to properly execute and is quite likely to fall through.

## How far along is the game?

Not very. I have a lot of the Engine done, but the game itself is still in the very early stages of development. I don't have a lot planned out just yet, but I do have a lot of ideas that I am very excited to implement. Progress of what I do know is planned can be found in the [Projects](https://github.com/users/GamingLiamStudios/projects/1/views/1) tab.
