# project path

## place link sign
<!-- keep the format -->
>[!NOTE]
>Symbol to mark web external links [![alt text][1]](./README.md)
<!-- keep the format -->
<!-- make folder and download the link sign vai curl -->
<!-- mkdir -p img && curl --create-dirs --output-dir img -O  "https://raw.githubusercontent.com/MathiasStadler/link_symbol_svg/refs/heads/main/link_symbol.svg"-->
<!-- Link sign - Don't Found a better way :-( - You know a better method? - send me a email -->
[1]: ./img/link_symbol.svg
<!-- keep the format -->

## init rust project

## Create a new rust based project inside the previously generated folder and update the rust binary's system wide to the last stable version
<!-- -->
```bash <!-- markdownlint-disable-line code-block-style -->
touch README.md \
&& ln -s README.md README \
&& cargo init "." \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& mkdir examples \
&& cp src/main.rs examples/example.rs \
&& sed -i -e 's/world/example/g' examples/example.rs \
&& rustup show \
&& rustup check \
&& rustup toolchain uninstall stable \
&& rustup toolchain install stable \
&& rustup update  --force \
&& rustup show \
&& mkdir tests \
&& rustup override set stable \
&& rustup show |sed -n '/active toolchain/,/^$/p'
```
<!-- keep the format -->
## Show which toolchain is active
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
rustup show
# or better
rustup show |sed -n '/active toolchain/,/^$/p'
```
<!-- keep the format -->
## Set/switch  rust toolchain - switch stable to nightly and back
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
rustup override set nightly
#or
rustup override set stable
```
<!-- keep the format -->
## Show rustc version verbose
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
rustc --version --verbose
```
<!-- keep the format -->
>&nbsp;[!TIP]
> Make sure the stable toolchain is activated
<!-- keep the format -->
>[!TIP] Markdownlint - Rules inside files can be enabled, disabled
> <!-- markdownlint-disable-next-line --> [![alt text][1]](https://github.com/DavidAnson/markdownlint)
<!-- keep the format -->
## Cargo clean - Remove artifacts that cargo has generated in the past
<!-- keep the format -->
- -v, --verbose...               Use verbose output (-vv very verbose/build.rs output)
<!-- markdownlint-disable-next-line -->
--color <WHEN>             Coloring [possible values: auto, always, never]
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
cargo clean -vv --color always
```
<!-- keep the format -->
## Add crates for project
<!-- keep the format -->
- egui [![alt text][1]](https://crates.io/crates/egui)
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
cargo add egui
cargo add egui_plot
cargo add env_logger
cargo add eframe
```
