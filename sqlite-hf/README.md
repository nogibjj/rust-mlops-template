## Analyze lyrics of a song with Rust and Hugging Face and SQLite


![hugging-face](https://user-images.githubusercontent.com/58792/215151354-30cadb7d-c972-479d-afc7-030bd684d4d2.png)



```bash
 cargo run -- candidates
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/sqlitehf candidates`
1
rock
2
pop
3
hip hop
4
country
5
latin
```

```bash
cargo run -- lyrics    
   Compiling sqlitehf v0.1.0 (/Users/noahgift/src/rust-mlops-template/sqlite-hf)
    Finished dev [unoptimized + debuginfo] target(s) in 0.76s
     Running `target/debug/sqlitehf lyrics`
Lyrics lyrics.txt
Uh-uh-uh-uh, uh-uh
Ella despidió a su amor
```

```bash
@noahgift ➜ /workspaces/rust-mlops-template/sqlite-hf (main ✗) $ cargo run -- classify
   Compiling sqlitehf v0.1.0 (/workspaces/rust-mlops-template/sqlite-hf)
    Finished dev [unoptimized + debuginfo] target(s) in 8.76s
     Running `target/debug/sqlitehf classify`
Classify lyrics.txt
rock: 0.06948944181203842
pop: 0.27735018730163574
hip hop: 0.034089818596839905
country: 0.7835917472839355
latin: 0.6906086802482605
```

