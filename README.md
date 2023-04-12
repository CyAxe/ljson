# rust-json
Lua Module to parse JSON written in Rust


```lua
local ljson = require("ljson")

ljson.decode('{"name": "Khaled"}') // Lua Table {name = "Khaled"}
ljson.encode({"name": "Khaled"}) // Lua String '{name = "Khaled"}'
```
