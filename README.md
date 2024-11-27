[![crates.io](https://img.shields.io/crates/v/cliff3-util.svg)](https://crates.io/crates/cliff3-util)
[![docs](https://docs.rs/cliff3-util/badge.svg)](https://docs.rs/cliff3-util)
[![Cargo test](https://github.com/JoonHoSon/rust-util/actions/workflows/cargo_test.yml/badge.svg)](https://github.com/JoonHoSon/rust-util/actions)

# openssl 설정

## Windows(x86 /x64)

[stack overflow 참고](https://stackoverflow.com/a/61921362)

* `vcpkg` 설치

```shell
c:\> git clone https://github.com/Microsoft/vcpkg
c:\vcpkg> ./bootstrap-vcpkg.bat 
```

* `openssl` 설치

```shell
c:\vcpkg> vcpkg.exe install openssl-windows:x[86|64]-windows
c:\vcpkg> vcpkg.exe install openssl:x[86|64]-windows-static
c:\vcpkg> vcpkg.exe integrate install
```

* 윈도우 환경변수 설정
    * `OPENSSL_LIB_DIR` 경로 추가
    * `OPENSSL_INCLUD_DIR` 경로 추가
    * `PATH`에 `c:\vcpkg\installed\x[86|64]-windows\bin` 추가(**중요**)

## Linux(Ubuntu 기준)

```bash
$ sudo apt update
$ sudo apt install build-essential pkg-config libssl-dev
```

# 변경 사항

## v0.2.3

### date_util

- get_latest_day 추가
- get_week_start_end 추가

## v0.2.2

### date_util 추가

- local_datetime_to_utc 추가
- utc_datetime_to_local 추가

## v0.2.1

### io_util 추가

- generate_path_string 추가
- generate_path 추가