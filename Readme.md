```
___________________
\                  \
 |   RHYMECRAFT     |
/                  /
| A TINS 2016 Game |
\        by        \
 |    SiegeLord     |
/__________________/
```
# Goal

Defeat all monsters! Summon allies using your rhyming magic!

HINT: It's possible to guess the correct poems, but if you're stuck, read data/spells.cfg!

# Controls

* Left mouse button - Select
* Right mouse button - Order

# Compilation

You should be able to compile it by getting a nightly Rust, installing Allegro 5.2.0 and then running `cargo run`.

On Windows, download Allegro 5.2.0 windows binaries into a sub-directory called `allegro` (it'll contain the `include` and `lib` directories) and then run `build_windows_msys.sh` from the MSYS shell (using MSVC linker will work too, so examine the script for what needs to be done). The resulting binary will be inside the `target/release` directory. If combined with the DLLs you downloaded, it should work great.

# Rules

## Genre requirements

*Craftsmanship: The game is centered around crafting items out of components.*

You, err, craft your allies out of poetic words!

## Artistic requirements

### Include dialog in poetic form (rhymed couplets, limerick, haiku) as much as you can

The spells to summon allies rhyme.

### Include snow

Snow is what you walk on.

## Technical requirements

### Path finding

Path finding is how units find their way around. I implemented A*.

### Use unicode to display non-english characters (e.g. russian, japanese). If you don't know any of these languages, that doesn't matter. Just use a phrase.

The spells use sprinkled unicode.

## Bonus rules

### Act of Formaldehyde

Unused.

## Data attributions.

a4_font.tga is a modified version of the same in Allegro.


## License

GPL-v3
