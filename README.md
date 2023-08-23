# my wiki

> A Box for Divertissement

There was no static site generator that pleases my fussy palate. So, I made it based on [markdown-rs](https://github.com/wooorm/markdown-rs). The markup language is slightly different from CommonMark, so I named my own version of markdown as `onnurmark`. The name is reminiscent of both 'on your mark' and my nickname. The features that are (going to be) implemented are as follows:

- [x] theorem box
- [x] multi-language support
- [ ] internal link support (including auto language fallback)
- [x] prevent mathjax to load if there is no math expression
- [ ] add icon in sidebar
- [ ] more elaborate (un)ordered list (*just like the latex's one*)
- [ ] fix a bug of ignoring emphasis, strong, etc. if following character is non-ascii unicode (e.g. `*무엇*이`) due to byte-wise parser
- [ ] render `` `asdf' `` to `‘asdf’`
- [x] syntax highlighting
- [ ] table of contents
- [ ] use local latex renderer instead of mathjax
- [ ] search feature
- [ ] make links that point to non-existing document red