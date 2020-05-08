# BunnyCDN CLI 🐰

The unofficial CLI for BunnyCDN 🐰

Note: storage API is the only one implemented so far, others coming soon! 

[BunnyCDN library](https://github.com/publicarray/bunnycdn)

## Install

```sh
cargo install bunnycli
```

## Requirements

 1. Have a bunnyCDN account
 2. An API Key. You can find this in our Dashboard in the My Account section.
 3. A Storage API key, You can find this in our Storage Zone, the FTP password is the API Key.

## Example Usage
```sh
./bunnycli storage --login storage_zone_name
./bunnycli storage --upload '/file/path/file.txt' 'server/path/file.txt'
./bunnycli storage --download '/save/file/path/file.txt' 'server/path/file.txt'
./bunnycli storage --info '/' | jq
./bunnycli storage --remove 'server/path/file.txt'
```

## Configuration

`~/.config/bunnycli.tml`

```toml
[storage_zone]
api_endpoint = "https://storage.bunnycdn.com"
name = "Your Storage Zone Name"
```

This is my first cli crate, please be gentle
