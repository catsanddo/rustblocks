# Rustblocks
## A dwm status bar program
Rustblocks is inspired by dwmblocks written entirely in Rust.
It aims to be easier to configure; 
instead of having to edit a header file users can add blocks via a `.rustblocks.toml` 
file located in their home directory.
An example file is included for reference.

### What it can do
- Custom blocks
- Varying update intervals
- Custom deliminator between blocks

### What it can't
Currently it lacks the ability to add custom signals to blocks.
Dwmblocks has a feature where blocks can be manually updated via a custom signal.
I would like to implement this feature in rustblocks, but I currently have no idea how to.
Also it has no support for color.


### Configuration
The example configuration file looks like this:

	deliminator = "|"

	[[block]]
	command = "date '+%b %d (%a) %I:%M%p'"
	interval = 60

The deliminator is the seporator between blocks and can be set to just about anything.
Each block should go under a new `[[block]]` and must contain the command to be called,
ant the interval in seconds it is updated at.
You can add as many blocks as you like after that.