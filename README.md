# Ricochet
Don't wanna read my explanation about why this thing exists? Go read the [wiki](https://github.com/CaffeinatedOpe/ricochet/wiki)
## A no-thrills, lo-frills link "shortener" taking the worst aspects of the Unix and K.I.S.S. philosophies to heart.
Ricochet was born out of a mix of disdain for external databases in a homelab, and the incredibly unintelligent mindset of "nah, I could do it better".
## Ok, nice buzzwords, what's the point?
Shlink exists, but I didn't like the requirement of an external db, or the off-site configuration page. Wasn't for me, and I didn't happen upon anything else I liked, so I made my own. It would also need to be stateless and cluster-friendly, which was iffy for some other options I found.  
My main use case was going to be (semi)permanent qr codes or nfc tags, and I wanted a way to "reprogram" them after installation. A link shortener makes sense for that, and I wouldn't be changing the links too much, so configuring them in the kubernetes ConfigMap makes sense. Rust was chosen out of my own interest in learning it... which was a bit of a [mistake.](https://caffeinatedope.net/blog/0005) But it's done, now we're here.
## Workings:
- Built in rust
- Actix-web for redirection
- serde for toml parsing (if needed)
- standard libs for env, filesystem parsing, etc.
## Usage:
Either install the docker container or clone the repo and build it. Run your compiled program directly via
```ricochet -c config.toml```  
(make sure your toml config exists, see either the default toml file or the wiki for config info).  

Run your program cooler and better with Docker:  
```docker run -v config.toml:/config/config.toml ghcr.io/caffeinatedope/ricochet-dev```

Kubernetes works great, see the documentation for configuration info, and the examples for example usage.
## Building:
To build, simply clone the repo, open it in a terminal, and run `cargo build`.

To build the docker container, you'll need to run `cargo build --target x86_64-unknown-linux-musl` first, then `docker build . --file ci/Dockerfile`.