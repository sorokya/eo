---
inject: true
to: src/net/packets/<%= type %>/mod.rs
after: 
skip_if: <%= family %>
---

pub mod <%= family %>;