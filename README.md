# Metropolis
![crates.io](https://img.shields.io/crates/v/metropolis.svg) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) ![doc.rs](https://docs.rs/metropolis/badge.svg?version=0.9.0)
## What is it and what is it for?

Metropolis is an easy to use high level graphics renderer written in rust, utilizing [vulkano](https://crates.io/crates/vulkano) and [winit](https://crates.io/crates/winit),
I still have some work to do on it and I am currently still developing it and would love community input.
Later I hope to develop a game engine using it but first I'll finish the renderer.
## How to install it?

### you can use cargo(the preferable and much easier and safe way):
```console
:~$ cargo install metropolis
```
### then install the vulkan required dependencies:
### if you have linux debian/ubuntu:
```console
:~$ apt install libvulkan1 mesa-vulkan-drivers vulkan-utils
```
### if you have fedora:
```console
# dnf install vulkan vulkan-info
```
### if you have arch linux:
```console
# pacman -S vulkan-radeon lib32-vulkan-radeon
```
### if you have mac:
```console
:~$ xcode-select --install
:~$ brew install cmake
```
### for windows just install ninja.

## Usage

Add the following to your Cargo.toml:
```rust
[dependencies]
metropolis = "0.9.0"
```
First use import the crate:
```rust
extern crate metro;  
use metropolis::*; 
use metropolis::color::*; 
//if you want some math functions use math as well
use metropolis::math::*;
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
 	     	fill(rgb(255,0.1.3));
	       	ellipse(400,posy,200,100);
	};
```
Finally use the show() function to run the whole thing:
```rust
	show(draw);
}
```
If you noticed - this program displays gravity working on an ellipse

## release notes:
### a patch version to fix the image module:
I finally fixed the image module and now it could be used to place a png image wherever you want in the page in the same size as it was(the resize is on you...)
### former versions release notes:
#### 0.8.0
I improved the text fps by a bit, added a Vector struct that allowes for some linear algebra related calculations, added from trait,display trait, and debug trait to color. </br>changed image functions: now only takes png images, and displays the whole image, I still have problems with it that I'm fixing.</br>changed the curve to work with bezier by default(beacuse of problems with te catmull rom chain)
#### 0.7.0
added the possiblity to use keyboard events see the examples:key_event and key_event_glob. I added mouse position getters and mouse scroll delta getters.
#### 0.6.0
you can now use a public mutithreading safe canvas struct, the matching example is called canvas_struct
#### 0.5.1 
fixed a bug that caused the text vertecies to not be cleared at he end of each iteration of the draw function
#### 0.5.0
some of the math functions were deprecated due to community feedback:sin,cos,tan,abs
I added an image module that allows you to load an image and display it(see the example for more details)
#### 0.4.1
fixed the text module slowdown for the non-text using canvases
added FPS unsafe static
added WIDTH/HEIGHT unsafe statics
#### 0.4.0
1 - added a text module - uses text() and textSize()
2 - added abs() and absf() functions to math
#### 0.3.3
1 - changed map to recieve generic variable
2 - added quad and square functions
#### 0.3.2
ported to vulkano 0.16, fixed the problem with the unclosing window!
#### 0.3.1:
there is a bezier curve, 2 function - one for vertex(4 x's and y's) and one for a chain(should have amout of values of 4+3*i such as 4,7,10,13...)
#### 0.3.0:
1 - there is a mapping function called map
2 - there is a factorial function called factorial
3 - there is a function called linspace that create evenly spaced floats between two numbers
4 - there are curves - using the catmull rom chain algorithm there is are functions to create a curves: curve, curveVertex, catmull_rom_chain 

### If you want to checkout the crate further that you should take a look in the [examples](https://github.com/GuyL99/metropolis/tree/master/examples) folder.
## Currently being developed:
1)dynamic line width.</br>
2)page elements and multicanvas module</br>
3)making the static mut into a lazy_static(in development).</br>
4)3D.</br>
5)HTML type file parsing into elements.</br>
6)anithyng else from community feedback!</br>

# License 
This crate is primarily distributed under the terms of the MIT license
See  [LICENSE-MIT](https://github.com/GuyL99/metropolis/blob/master/LICENSE) for details.
