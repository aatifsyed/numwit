# numwit

Witness types and operations for numbers which are [`Positive`] or [`Negative`], but not zero.

| Operation | LHS        | RHS             | Output     | Assignable? |
| --------- | ---------- | --------------- | ---------- | ----------- |
| Add       | `Positive` | `Positive`      | `Positive` | Yes         |
|           | `Negative` | `Negative`      | `Negative` | Yes         |
|           | `Positive` | `Negative`      | ?          | No          |
|           | `Negative` | `Positive`      | ?          | No          |
|           | `Positive` | `impl Unsigned` | `Positive` | Yes         |
|           | `Negative` | `impl Unsigned` | ?          | No          |
| Sub       | `Positive` | `Positive`      | ?          | No          |
|           | `Negative` | `Negative`      | ?          | No          |
|           | `Positive` | `Negative`      | `Positive` | Yes         |
|           | `Negative` | `Positive`      | `Negative` | Yes         |
|           | `Positive` | `impl Unsigned` | ?          | No          |
|           | `Negative` | `impl Unsigned` | `Negative` | Yes         |
| Mul       | `Positive` | `Positive`      | `Positive` | Yes         |
|           | `Negative` | `Negative`      | `Positive` | No          |
|           | `Positive` | `Negative`      | `Negative` | No          |
|           | `Negative` | `Positive`      | `Negative` | Yes         |
|           | `Positive` | `impl Unsigned` | ?          | No          |
|           | `Negative` | `impl Unsigned` | ?          | No          |
| Div       | `Positive` | `Positive`      | `Positive` | Yes         |
|           | `Negative` | `Negative`      | `Positive` | No          |
|           | `Positive` | `Negative`      | `Negative` | No          |
|           | `Negative` | `Positive`      | `Negative` | Yes         |
|           | `Positive` | `impl Unsigned` | `Positive` | Yes         |
|           | `Negative` | `impl Unsigned` | `Negative` | Yes         |
| Neg       | `Positive` |                 | `Negative` |             |
|           | `Negative` |                 | `Positive` |             |

License: MIT or Apache-2.0
