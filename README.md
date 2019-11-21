# Metropolis
![crates.io](https://img.shields.io/crates/v/metropolis.svg) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) ![doc.rs](https://docs.rs/metropolis/badge.svg?version=0.1.1)
## What is it and what is it for?

Metropolis is an easy to use high level graphics renderer written in rust, utilizing [vulkano](https://crates.io/crates/vulkano) and [winit](https://crates.io/crates/winit),
I still have some work to do on it and I am currently still developing it and would love community input.
Later I hope to develop a game engine using it but first I'll finish the renderer.
## How to install it?

### you can use cargo(the preferable and much easier and safe way):
```console
:~$ cargo install metropolis
```

## Usage

Add the following to your Cargo.toml:
```rust
[dependencies]
metropolis = "0.1.1"
```
First use import the crate:
```rust
extern crate metro;  
use metro::*; 
use metro::color::*; 
//if you want some math functions use math as well
use metro::math::*;
```

Then you use the funcion size that creates a canvas(I wwould suggest to save height and width as variables so you can use them later
```rust
fn main(){                                                             
	let height = 600;
	let width = 800;
	size(width,height);
```
Next comes the setup(here I declare the varibles I will be using insode the looped function):
```rust
	let mut spd = 0;
	let mut acc = 1;
	let mut posy = 0;
	background(grayscale(220));
```
Next comes the draw function, this function gets looped over so what's in it should be decided accordingly
```rust
    	let draw =move || {
        	spd+=1;
        	if posy+50< height{
	            posy+=spd;
        	}
 	     	fill(rgb(255,0.1.1));
	       	ellipse(400,posy,200,100);
	};
```
Finally use the show() function to run the whole thing:
```rust
	show(draw);
}
```
If you noticed - this program displays gravity working on an ellipse

### If you want to checkout the crate further that you should take a look in the [examples](https://github.com/GuyL99/metropolis/tree/master/examples) folder.
## Currently being developed:
1)dynamic line width.
2)up the circle and ellipse efficiency.
3)add more drawing functions.
4)add a text module.
5)adding unit tests.
6)3D.
7)anithyng else from community feedback!

# License 
This crate is primarily distributed under the terms of the MIT license
See  [LICENSE-MIT](https://github.com/GuyL99/metropolis/blob/master/LICENSE) for details.
