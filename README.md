# vc
Command line value conversions

First iteration is just going to convert between number systems used by programmers.  For exampple, show the different represntations for `1234`:

```
$ vc 1234
Decimal     Hex          Octal        ASCII    Binary
1234        0x000004D2   0o00002322   Ò        0b00000000000000000000010011010010
```

Or, you can put in a hex number:

```
$ vc 0x1234
Decimal     Hex          Octal        ASCII    Binary
4660        0x00001234   0o00011064   4        0b00000000000000000001001000110100
```
