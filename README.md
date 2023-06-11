# pascii

is a ripoff of jp2a that does two things:

- is very fast: 2ms for 32x32 images with full color (enable rayon feature for images > 4000px)
- doesnt repeat ansi escape sequences (no `\e[34;5mc\e[34;5md\e[34;5me`, just `\e[34;5mcde`)

see [output.ansi](https://github.com/bend-n/pascii/blob/master/output.ansi) for a example.