## Rust Program Basics
### File name convention
Rust 파일은 `.rs`라는 확장자를 사용함. 두 단어 이상 사용하고 싶으면 `_`로 분리(예: `hello_world.rs`)
### Compile & Run
```
$ rustc main.rs
$ ./main
```

## Cargo
Rust의 빌드 시스템이자 패키지 관리자.
### Create Project 
```
$ cargo new hello_cargo
```
```
.
├── Cargo.toml
└── src
    └── main.rs
```
### Cargo.toml
```
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

* `[packege]`: 패키지 구성요소들로 Cargo가 프로그램을 컴파일하는데 필요한 정보들이 적혀있음.
* `[dependencies]`: 프로젝트가 가지는 의존성 정보가 적혀있음.

각 섹션은 `[]` 이후 부터 새로운 헤더가 나오기 전까지 유지됨.

### Build & Run
```bash
$ cargo build # 실행 파일과 라이브러리 생성
$ cargo run   # 빌드 후 프로그램 실행
$ cargo check # 컴파일 가능 여부를 빠르게 검사
```

`example`
```bash
~/hello_cargo$ cargo build
   Compiling hello_cargo v0.1.0 (/home/kmwook/Rust-Exercise/ch1/hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.99s
~/hello_cargo$ tree
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
        ├── build
        ├── deps
        │   ├── hello_cargo-05716a7b81a7e9a0
        │   └── hello_cargo-05716a7b81a7e9a0.d
        ├── examples
        ├── hello_cargo
        ├── hello_cargo.d
        └── incremental
            └── hello_cargo-1ai9p60ma1lmy
                ├── s-hlbv10aom9-0g2dujn-e8kgjtys2wiyxd0oubzktwscv
                │   ├── 2a4d34fi07ovifzyarz15b2oh.o
                │   ├── 4428m9e2sgxbubfr6p9w4wgu0.o
                │   ├── 8rkrdkbj1zo92n9yech0lqavc.o
                │   ├── bccsp8akzciarxadjgfhb01m7.o
                │   ├── ci4nd6hvprba3kl3fr2phu9yh.o
                │   ├── dep-graph.bin
                │   ├── e28x20kc3o0e9i3qevaxu9hzf.o
                │   ├── query-cache.bin
                │   └── work-products.bin
                └── s-hlbv10aom9-0g2dujn.lock

10 directories, 18 files
~/hello_cargo$ ./target/debug/hello_cargo 
Hello, world!
~/hello_cargo$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/hello_cargo`
Hello, world!
```

#### Building for Release
```bash
$ cargo build --release
```
최적화를 적용하여 컴파일함. 이 명령어를 실행하면 `target/debug` 대신 `target/release` 디렉터리에 실행 파일이 생성됨.
최적화 기능을 사용하면 Rust 코드의 실행 속도가 빨라지지만, 프로그램 컴파일 시간이 길어짐. 
따라서 두 가지 프로필이 제공됨.

1. 개발 환경에서 빠르게 자주 재빌드할 때 사용하는 프로필
2. 최종 사용자에게 제공할 프로그램을 빌드할 때 사용하는 프로필

반복적인 재빌드 없이 최대한 빠르게 실행되도록 설계됨.
