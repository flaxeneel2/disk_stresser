# Disk stresser
A multithreaded disk write program written in rust.

Flags:
```
    -b, --bytes <BYTES>              Number of bytes to write. Can be provided with K/M/G suffixes
                                     to write in kb, mb and gb respectively. ex: -b 4G
                                     
    -c, --chunk-size <CHUNK_SIZE>    The size of every chunk that is written onto the drive, can be
                                     provided with K/M/G suffixes.
                                     
    -d, --delete-file                Delete the file after the test is done.
    -h, --help                       Print help information
    -t, --threads <THREADS>          Number of threads to write with
    -V, --version                    Print version information

```

I just made this because it was a weekend, and I was bored.

I am not responsible for any damage done. Use at own risk.

Dependencies:
* Clap