# Puffy Blowhog DNS management

Terraform got you down? Just want to manage your cloudflare DNS without the fuss?
Well, you've come to the right place! Puffy Blowhog enables simple Cloudflare DNS management.

WARNING: this is pre-alpha software! It does not work!

## Configuration

Puffy Blowhog uses TOML configuration files.

```toml
[zone]
id = "2da441a282c0ba66367d891ec4286578"
name = "minecraft.nexus"
restrictive_email = true

[[records]]
name = "@"
type = "AAAA"
content = "100::"
proxied = true

[[records]]
name = "www"
type = "AAAA"
content = "100::"
proxied = true
```

This allows it to be easy-to-use and concice.
