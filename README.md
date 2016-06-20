A proof-of-concept implementation of (parts of) the Rust Standard Library for Nintendo 3DS homebrew. Highly experimental and not recommended for general use.

Issues:

* This library is heavily incomplete at best. Expect many features to not be present and expect the ones that are there to probably be broken in odd ways.
* Panicking is currently implemented as an endless loop. No unwinding, no error messages, just your program locking up and requiring a hard reboot.
* The Rust project hasn't bestowed the power of custom preludes upon the masses yet, so the std prelude has to be manually imported into every module you want to use it in.

The future of this library and what role it might play in the ecosystem for Rust 3DS homebrew is uncertain. Any comments or PRs are welcome.
