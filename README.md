# Todo-rs

> Todo TUI written in Rust

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

## Timestamp Format

There are 3 different available format which are:

- `year-month-date hour:minute:second` (***####-##-## ##:##:##***)
- `hour:minute:second` (***##:##:##***)
- `year-month-date` (***##-##-##***)

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
