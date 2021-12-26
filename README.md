# FILE API

A simple file api made using rust 📂

| Folder Name | Function                                         |
| ----------- | ------------------------------------------------ |
| api         | contains code for file-api                       |
| files       | all files will be saved in that folder           |
| test-site   | a simple website for testing the upload endpoint |

Run server to get a server at http://localhost:8080 with two routes:

- `POST /upload` \- you can upload a file here
- `GET /files/*` - download the uploaded files

The file is saved using a random id as a name, with the extension. The name is logged, so you can see that it worked.

The max file-size is configured at ~100 MB.

Thanks @zupzup for creating [warp-upload-download-example](https://github.com/zupzup/warp-upload-download-example/) (The most part of this project is copied from `warp-upload-download-example` repo)

## SETUP

1. Using Rust & Cargo

   You need `Rust` & `Cargo` installed to get started

   move to the folder `api` and run

   ```bash
   $ cargo run
   ```

2. Using Docker

   You need `Docker` installed for this

   ```bash
   $ docker build -t file-api
   ```

   ```bash
   $ docker run -p 8080:8080 file-api
   ```

   
