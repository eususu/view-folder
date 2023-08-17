# view-folder
> Command Line Tool for Duplicate Folder(Sturcture) and Link file with extension

[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE-APACHE) [![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

# About

view-folder는 obsidian.md을 사용해보려다 필요에 의해서 만들어졌습니다.
markdown 파일이 들어 있는 각 프로젝트의 디렉터리를 등록해서 사용하려고 했는데, markdown이 아닌 파일 때문에 불편함이 있었습니다.

각종 디렉터리에 있는 필요한 파일들만 symbolic 링크를 한 곳에 등록해서 쓰자는 아이디어에서 시작되었습니다.

```bash
% vf --help
view-folder 0.1.0
belovian@gmail.com

USAGE:
    vf [OPTIONS] --output <output directory> --extension <file extension>... [--] [input file or directory]...

ARGS:
    <input file or directory>...    specify input file or directory

OPTIONS:
    -d, --debug
            Turn debugging information on

    -e, --extension <file extension>...
            specify a file extension for include

    -h, --help
            Print help information

        --hard-link
            use hard link instead of symbolic

    -o, --output <output directory>
            specify output directory, if not exist, make it automatically

    -V, --version
            Print version information
```
