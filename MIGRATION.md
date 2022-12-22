# Migration Guide

## `0.2.*` -> `0.3.*`
> This release is action packed so be prepared!

Steps (This order is recommended so lsp doesn't scream at you lol)
1. Add the prelude! This new release includes traits! `use hyprland::prelude::*;`
2. [Update your functions to structs](#Zero-Point-Three)





## More in-depth steps

### Zero Point Three

#### All the data fetchers have been updated so time to do fix them!

| Notation Type! | Notation                                                     |
|----------------|--------------------------------------------------------------|
| Old notation   | `asynchronous::get_something()` / `blocking::get_something()`|
| New Notatin!! | `Something::get_async().vec()` / `Something::get().vec()`    |
> The `.vec()` is required because data is now stored as a struct!


