# BunnyCDN CLI 🐰

Note: storage API is the only one implemented so far, others coming soon!

## Requirements

 1. Have a bunnyCDN account
 2. An API Key. You can find this in our Dashboard in the My Account section.
 3. A Storage API key, You can find this in our Storage Zone, the FTP password is the API Key.

```sh
./bunnycli storage --login api-test
./bunnycli storage --upload '/file/path/file.txt' 'server/path/file.txt'
./bunnycli storage --download '/save/file/path/file.txt' 'server/path/file.txt'
./bunnycli storage --remove 'server/path/file.txt'
```

This is my first cli crate, please be gentle
