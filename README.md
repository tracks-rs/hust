# Hust

Hust is an HTML-first way to embed Rust in HTML.

## This Crate

This crate exports a function called `preprocess_and_generate_rust_code`. It is meant to be used in your own proc macros.

To use hust without implementing your own macro, you should instead use the crate `hust-macro`.

## Example Usage

```
<h1>User</h1>
<div class="user">
  <%= &user.username %>
</div>
```

Or for more complex usage:

```
<h1>All Users</h1>
<div class="py-2">
  <%= &users.len().to_string() %> users found.
</div>
<% for user in users { %>
  <div class="user">
    <a href="/users/<%= &user.id.to_string() %>">
    <%= &user.username %>
    </a>
  </div>
<% } %>

<a href="/users/new" class="btn btn-primary">New User</a>
```