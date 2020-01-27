(module
  (type $t0 (func (param i32 i64 i32) (result i32)))
  (type $t1 (func (param i32 i32 i32 i32) (result i32)))
  (type $t2 (func))
  (import "wasi_unstable" "clock_time_get" (func $wasi_unstable.clock_time_get (type $t0)))
  (import "wasi_unstable" "fd_write" (func $wasi_unstable.fd_write (type $t1)))
  (func $_start (type $t2)
    i32.const 0
    i64.const 1000
    i32.const 100
    call $wasi_unstable.clock_time_get
    drop
    i32.const 0
    i32.const 8
    i32.store
    i32.const 4
    i32.const 6
    i32.store
    i32.const 1
    i32.const 0
    i32.const 1
    i32.const 20
    call $wasi_unstable.fd_write
    drop)
  (memory $memory 1)
  (export "memory" (memory 0))
  (export "_start" (func $_start))
  (data $d0 (i32.const 8) "Done!\0a"))
