# TXTree

# Behaviour
## Branching
Consider the input `o[o[o,o[o]],o]`
Here's the different results that can be achieved.

Default settings
```txtree
o
├╴o
│ ├╴o
│ └╴o
│   └╴o
└╴o
```

With `-f`
```txtree
  ┌╴o
  │ ┌╴o
  ├╴o
┌╴o
├╴o
o
```

With `-m`
```txtree
  ┌╴o
┌╴o
│ └╴o
│   └╴o
o
└╴o
```

With `-i`
```txtree
      o
    o╶┤
  o╶┤ │
  o╶┘ │
o╶┘   │
    o╶┘
```

With `-fi`
```txtree
  o╶┐  
o╶┐ │  
  o╶┤  
    o╶┐
    o╶┤
      o
```

With `-mi`
```txtree
  o╶┐
    o╶┐  
  o╶┘ │
o╶┘   │
      o
    o╶┘
```

With `-c`
```txtree
o╶┬╴o╶┬╴o
  │   └╴o╶─╴o
  └╴o
```

With `-fc`
```txtree
      ┌╴o
  ┌╴o╶┴╴o╶─╴o
o╶┴╴o
```

With `-mc`
```txtree
      ┌╴o
  ┌╴o╶┤
  │   └╴o╶─╴o
o╶┤
  └╴o
```

With `-ic`
```txtree
      o
    o╶┤
  o╶┤ │
  o╶┘ │
o╶┘   │
    o╶┘
```

With `-fic`
```txtree
  o╶┐  
o╶┐ │  
  o╶┤  
    o╶┐
    o╶┤
      o
```

With `-mic`
```txtree
  o╶┐
    o╶┐  
  o╶┘ │
o╶┘   │
      o
    o╶┘
```

┌─┬─┐
├─┼─┤
├╴│╶┤
└─┴─┘
