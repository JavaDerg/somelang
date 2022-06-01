# Identifier
```regexp
[a-zA-Z_]\w*
```

# Numeric
```regexp
(:?[\-+])?
\d+(?:\.\d+)?
(?:[ui](8|16|32|64)|f(32|64))?
```

# List
`( ... )`

# Match
`=`

# Chain
`.`

# Defines
`->`

# Operators
```regexp
(\.{2,3}|>=|<=|<<|>>|[+\-*/%!|&^<>])
```

# Generic
```regexp
\b'IDENTIFIER\b
```

# Char
```regexp
'(ESCAPECODES|[^'])'
```

# Strings
```regexp
"(ESCAPECODES|[^"]*)"
```
