```console
$ make release DOCKER=
$ ./main a b
args(1): ./main
args(2): a
args(3): b
```

#### Benchmark

- file: `/tmp/file` (871MB)

```console
$ ./main /tmp/file >| /dev/null
0.92s user 0.51s system 99% cpu 1.425 total

$ ./cr-cat /tmp/file >| /dev/null
0.85s user 0.39s system 101% cpu 1.223 total

$ cat /tmp/file >| /dev/null
0.00s user 0.20s system 99% cpu 0.203 total
```
