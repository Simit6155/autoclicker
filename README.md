# Rust Autoclicker

A simple autoclicker written in Rust.

- Clicks a set number of times or infinitely  
- Customizable delay between clicks

## Usage

1. Clone the repo:

git clone https://github.com/Simit6155/autoclicker.git


cd autoclicker


2. Build and run:

cargo run --release


3. Enter the delay in milliseconds.  
4. Enter the number of clicks (0 = infinite).  
5. Place your mouse where you want it to click.  
6. Stop infinite mode with `Ctrl+C`.

## Example

Enter Delay: 200
Enter limit of presses (0 = infinite): 5

Clicks 5 times with 200ms between clicks.

## Notes

- Works on **Windows** and Linux X11.  
- Doesn’t work on Wayland.  
- Use responsibly.
