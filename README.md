# LuaU Parser
An attempt to have a stable parser, easily modified, for the LuaU programming language. Will also have backwards compatibility to Lua by nature

## Currently Parses
Tables (including nested tables), numbers, strings, booleans, comments.

## Todo
Parse entire function bodies, storing the local identifiers of a function

Methods not defined in a table
```lua
local tab = {}

function tab.thing(one, two)

end
```

Locally defined methods
```lua
local tab = {}

local tab.t = function(a, b)

end

local otherFunction = function(a, b)

end
```

Implement [ROBLOX DOM types](https://github.com/rojo-rbx/rbx-dom#readme)

Basic mathematical functions (+-*/^%)

Increment and decrement (+= -=)