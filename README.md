# Todo-rs

> Todo TUI written in Rust. This is a rough work in progress, so all features
> and appearances are subjected to changes

## Demo

![](./assets/demo.gif)

## Usage

Main Screen:
- `a`: open add item prompt
- `d`: remove item
- `y`: toggle item
- `j`: scroll down
- `k`: scroll up
- `q` or `Ctrl + c`: quit the program
- `?`: toggle help display

Add Item:
- `Ctrl + d`: toggle expire date prompt
- `Enter`: accept input
- `Escape`: exit prompt

## Building

Clone the repository:

``` sh
$ git clone https://github.com/Z5483/todo-rs.git
```

Run the following the build:

``` sh
$ cargo build --release
```

The binary should reside in `target/release/todo-rs`
