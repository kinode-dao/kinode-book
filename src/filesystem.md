# Filesystem

Draft: 11/28/23

### Overview

The Uqbar filesystem is designed to be an extensible, performant and consistent way to persist data. It operates on 3 abstraction levels:

- **memory**: buffering writes and caching reads for efficiency, flushing to disk upon shutdown/buffers reaching their limits.

- **disk**: 2 central files exist on your local disk, `manifest.bin` and `WAL.bin`. The manifest is the descriptor of your files, where they exist, along with other metadata. The write ahead log (WAL) is used for consistency and recoverability. It also acts as a buffer for flushing to cold storage

- **cold storage**: content addressed object storage. Objects are stored by their blake3 hash, and can be stored both on local disk or in the cloud.

### Configuration
The filesystem is highly configurable, depending on your storage needs, system resources and preferences. When booting your node, you can supply a `.env` file, here's the `.env.example` file from the repo, with it's default values commented out.

```
# MEM_BUFFER_LIMIT=5242880        # 5mb
# READ_CACHE_LIMIT=5242880        # 5mb
# CHUNK_SIZE=262144               # 256kb
# FLUSH_TO_COLD_INTERVAL=60       # 60s
# ENCRYPTION=true                 # true
# CLOUD_ENABLED=false             # false

###  Example s3 config
# S3_ACCESS_KEY=minioadmin
# S3_SECRET__KEY=minioadmin
# S3_REGION=eu-north-1
# S3_BUCKET=uqbar
# S3_ENDPOINT=http://localhost:9000
```

[link to docker config]

### Benchmarks

### See also

- [link to VFS api Reference]
- [link to FS api Reference]
- [link to cloud setup, backup setup]
- [link to file-transfer app setup and usage]


