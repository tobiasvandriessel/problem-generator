/*!

The problem generator crate for TD Mk Landscape benchmark generation.

## Quick Start

### Library

To use problem_generator in your project, you can simply add problem_generator into your ```cargo.toml```:

```toml
[dependencies]
problem_generator = "^0.3.0"
```
*/

#[macro_use]
extern crate log;

extern crate structopt;
extern crate structopt_derive;

///The parent module for all the functional modules
pub mod problem;
