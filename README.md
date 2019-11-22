# Metropolis
![crates.io](https://img.shields.io/crates/v/metropolis.svg) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) ![doc.rs](https://docs.rs/metropolis/badge.svg?version=0.1.8)
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
## Documentation:
### Due to a bug in docs.rs the crate does not have a docs.rs documentation so I put the gist in here:
#### regular functions(to be used directly from use metro):
arc- create an arc from a circle, recieves the center of the circle and the radius and the degrees covered by the arc (360 degree arc is a full circle).
background- sets the background color(using the color struct).
circle-recieves the x and y of the center of the circle and the radius and builds it with them.
ellipse	-recieves the x and the y of the center of the ellipse and the width and height of the ellipse and creates it accordingly
fill-enables fill and receives the color of the fill(the struct color) and sets the fill color to be the color.
line-recieves the x and y of the top point and then the x and the y of the bottom point and creates a line between them.
noFill-disables fill on the canvas.
noStroke-disables stroke on the canvas.
point-recieves the x and the y and makes a small circle in the spot(size depends on strokeWeight).
rect-recieves the x and y of the top spot and then the width and height of the rectangle you want built.
show-this is the function used to run the animation
size-creates the canvas with the width and height sent to this function
stroke-enables stroke and receives the color of the stroke(the struct color) and sets the stroke color to be the color.
strokeWeight-sets the stroke weight(the width of lines and points)
triangle-recieves the x and y of the 3 points of the triangle and creates it based on them
#### functions from the color module:
grayscale-retrun Color sruct from grayscale values
rgb-retrun Color sruct from rgb values
rgba-retrun Color sruct from rgba values
#### functions from the math module:
cos-uses sin(90-alpha) in order to calculate cosine
deg-converts radians to degrees
rad-converts degrees to radians
sin-uses Taylor's series to determine sinus at the x, for now accepts f32, in the future also u/i
tan-uses sin(x)/cos(x) in order to calculate tan(x)
TWO_PI - a constant containing two pi
PI - a constant containing the value of pi
## Usage

Add the following to your Cargo.toml:
```rust
[dependencies]
metropolis = "0.1.8"
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
