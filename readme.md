# mem_prefetch

This crate provides a simple platform agnostic wrapper for memory prefetching.

## Cargo.toml

```toml 
[dependencies]
mem_prefetch = "0.1"
```

## Usage

``` rust
mem_prefetch::prefetch_read_data::<_, 3>(reference);  // most local
mem_prefetch::prefetch_write_data::<_, 3>(reference); // most local 

mem_prefetch::prefetch_read_data::<_, 2>(reference);
mem_prefetch::prefetch_write_data::<_, 2>(reference);

mem_prefetch::prefetch_read_data::<_, 1>(reference);
mem_prefetch::prefetch_write_data::<_, 1>(reference);

mem_prefetch::prefetch_read_data::<_, 0>(reference);  // least local
mem_prefetch::prefetch_write_data::<_, 0>(reference); // least local


// Or raw ptr variants:
unsafe {
    mem_prefetch::prefetch_read_data_raw::<_, 0>(ptr);
    mem_prefetch::prefetch_write_data_raw::<_, 0>(ptr);
}
```


## Features 

- `fallback`: Use fallback `ptr::read_volatile` for prefetching if no prefetch instruction is available. (Enabled by default).
- `nightly`: Use llvm intrinsics from `core_intrinsics` for prefetching.

## Notes
- read/write variants map to same instructions on `x86`.
