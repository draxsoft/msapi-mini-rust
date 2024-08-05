# Macsploit Rust Mini API

## Overview

Macsploit's Rust Mini API is an open-source API for interacting with Roblox exploits. It allows developers to send custom scripts to a local server, making it easier to manage and create custom user interfaces for Roblox exploits.

## Features

- **Send Scripts**: Send custom Roblox scripts to a local server.
- **Timeout Handling**: Uses standard TCP connection without explicit timeout, but can be adjusted as needed.
- **Default Port**: Uses port 5553 by default.

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/) installed. You can install Rust using [rustup](https://rustup.rs/).
2. Create a new Rust project or save the provided script in your existing project.

## Usage

1. **Define Your Script**: Update the `script` variable with your desired Roblox script.

```rust
let script = "loadstring(game:HttpGet(\"https://raw.githubusercontent.com/EdgeIY/infiniteyield/master/source\"))()";
```
