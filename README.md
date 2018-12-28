# vc
Command line math utility.  

Lets you convert between radixes.  For exampple, show the different represntations for `1234`:

```
$ vc -r 1234
Decimal     Hex          Octal        ASCII    Binary
1234        0x000004D2   0o00002322   Ò        0b00000000000000000000010011010010
```

Or, you can put in a hex number:

```
$ vc -r 0x1234
Decimal     Hex          Octal        ASCII    Binary
4660        0x00001234   0o00011064   4        0b00000000000000000001001000110100
```

With no arguments, you can use it as a calculator.  So far this is really just the pest crate sample code running expressions from command line args:

```
# On bash at least, make sure to put expressions with * operators in quotes
vc "2 + 9 * 10"
2 * 9 + 10 = 28
```
