---
inject: true
to: src/net/packets/<%= type %>/<%= family %>/mod.rs
skip_if: <%= action %>
after: 
---

mod <%= action %>;
pub use <%= action %>::<%= h.capitalize(action) %>;