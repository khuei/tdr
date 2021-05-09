# Todo-rs

> text-based user interface utility to organize your todo list

## Demo

![](./assets/demo.gif)

## Features

- Text-based user interface
- create, edit, and remove todo item
- display countdown until deadline
- multiple workspace support
- display number of in progress, late, and completed item

## Usage

Item Display:
- `w`: add workspace
- `-`: display workspace
- `a`: add item
- `e`: edit item
- `d`: remove item
- `y`: toggle item
- `r`: rename current workspace
- `J`: scroll workspace down
- `K`: scroll workspace up
- `j`: scroll down
- `k`: scroll up
- `q` or `Ctrl + c`: quit the program
- `?`: toggle help display

Add Item:
- `Ctrl + d`: toggle expire date prompt
- `Enter`: accept input
- `Escape`: exit prompt

Workspace Display:
- `Enter`: select workspace
- `e`: edit workspace
- `d`: remove workspace
- `j`: scroll down
- `k`: scroll up

Add Workspace:
- `Enter`: accept input
- `Escape`: exit prompt

## Timestamp Format

There are 3 different available format which are:

- `year-month-date hour:minute:second` (***####-##-## ##:##:##***)
- `hour:minute:second` (***##:##:##***)
- `year-month-date` (***####-##-##***)

## Installation

Run the following to install:

```
$ cargo install todo-rs
```

## Build From Source

Clone the repository:

``` sh
$ git clone https://github.com/Z5483/todo-rs.git
```

Build:

``` sh
$ make
```

Install:

``` sh
$ make install
```

The default install prefix is `/usr/local`, you can change it by setting the
`PREFIX` variable at the beginning of the above command.
