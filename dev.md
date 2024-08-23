# Notes

Current instructions are:

- OpReturn
- OpConstant
- OpLongConstant
  - Taking 3 bytes for the constant offset (constant index in constant stack)

## Optimizations

- Could do pointer arithmatic instead
  - For all the offset calculating etc
- Could also use rust inbuilt types
  - But making my own to learn the whole process
