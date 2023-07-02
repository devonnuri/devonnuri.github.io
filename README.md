# my wiki

> A Box for Divertissement

There was no static site generator that pleases my fussy palate. So, I made it based on [markdown-rs](https://github.com/wooorm/markdown-rs). The features that are (going to be) implemented are as follows:

- [ ] theorem box
- [ ] multi-language support
- [ ] internal link support (including auto language fallback)
- [ ] prevent mathjax to load if there is no math expression
- [ ] add icon in sidebar
- [ ] more elaborate (un)ordered list (*just like the latex's one*)
- [ ] fix a bug of ignoring emphasis, strong, etc. if following character is non-ascii unicode (e.g. `*무엇*이`) due to byte-wise parser
- [ ] render `` `asdf' `` to `‘asdf’`