# pascii

is a ripoff of jp2a that does two things:

- is very fast: 2ms for `32x32` images with full color (enable rayon feature for images > `4000px`)
- doesnt repeat ansi escape sequences (no `\e[34;5mc\e[34;5md\e[34;5me`, just `\e[34;5mcde`)

pascii takes `91623` characters(measured with `wc -c`) and 5ms to draw this `250x166` image,<br>
wheras jp2a takes `115905` characters and 14ms, a whopping `24282` characters more.

[![image](https://raw.githubusercontent.com/bend-n/pascii/master/.github/output.png)](https://github.com/bend-n/pascii/blob/master/.github/output.ansi "ansi")
