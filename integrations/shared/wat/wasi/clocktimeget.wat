;; Attributions to Chris Whealy
;; https://github.com/ChrisWhealy/clock_time_get

(module
  (type $__wasi_clockTimeFnType (func (param i32 i64 i32)     (result i32)))
  (type $__wasi_fdWriteFnType   (func (param i32 i32 i32 i32) (result i32)))

  ;; Host functions provided by WASI
  (import "wasi_unstable" "clock_time_get" (func $wasi_clock_time_get (type $__wasi_clockTimeFnType)))
  (import "wasi_unstable" "fd_write"       (func $wasi_fd_write       (type $__wasi_fdWriteFnType)))

  ;; Line feed character
  (global $line_feed i32 (i32.const 10))

  ;; Upper and lower nybble masks
  (global $0xF0 i32 (i32.const 240))
  (global $0x0F i32 (i32.const 15))

  ;; Memory offsets
  (global $i64_bin_loc i32 (i32.const 16))
  (global $i64_str_loc i32 (i32.const 24))
  (global $fd_write_data_loc i32 (i32.const 44))

  (memory (export "memory") 1
    ;; 00 -> 15  16b ASCII character lookup table
    ;; 16         8b System clock value - binary
    ;; 24        16b System clock value - ASCII string
    ;; 44         8b fd_write output string offset + length
    ;; 52         4b Number of bytes written by fd_write
  )

  ;; ASCII lookup table where each character occurs at its corresponding offset
  (data (i32.const 0) "0123456789abcdef")

  ;; Increment/decrement functions
  (func $incr (param $val i32) (result i32) (i32.add (local.get $val) (i32.const 1)))
  (func $decr (param $val i32) (result i32) (i32.sub (local.get $val) (i32.const 1)))

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; The nybble value is used as the offset at which the corresponding ASCII character can be found
  ;;
  ;; Offset using upper nybble = byte AND'ed with 0xF0 then shifted right 4 places
  ;; Offset using lower nybble = byte AND'ed with 0x0F
  (func $upper_nybble_to_char (param $b i32) (result i32) (i32.load8_u (i32.shr_u (i32.and (local.get $b) (global.get $0xF0)) (i32.const 4))))
  (func $lower_nybble_to_char (param $b i32) (result i32) (i32.load8_u (i32.and (local.get $b) (global.get $0x0F))))

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Convert the 8 bytes of an i64 into a 16 character ASCII hex string.
  ;; Input  : []
  ;; Output : []
  ;;
  ;; This function does not accept arguments and does not "return" a value because globally referenced memory locations
  ;; are used instead
  (func $i64_to_hex_str
    (local $str_loc     i32)
    (local $loop_offset i32)
    (local $this_byte   i32)
    (local $upper_char  i32)
    (local $lower_char  i32)

    ;; Set offset of current string byte to the output string's start offset
    (local.set $str_loc (global.get $i64_str_loc))

    ;; Since the i64 number is stored in little endian format, the highest order byte is stored at the furthest offset
    ;; from the start.  In other words, we must reverse the byte order in the i64 in order for it to print correctly.
    ;; See https://en.wikipedia.org/wiki/Endianness for more details
    (local.set $loop_offset (i32.add (global.get $i64_bin_loc) (i32.const 7)))

    ;; Working from right to left (little endian byte order), parse each byte of the i64
    (loop $next_byte
      (local.set $this_byte (i32.load8_u (local.get $loop_offset)))

      ;; Transform each half of the current byte into an ASCII chararcter
      (local.set $upper_char (call $upper_nybble_to_char (local.get $this_byte)))
      (local.set $lower_char (call $lower_nybble_to_char (local.get $this_byte)))

      ;; Store each ASCII char and bump the output offset
      (i32.store8 (local.get $str_loc) (local.get $upper_char))
      (local.set $str_loc (call $incr (local.get $str_loc)))

      (i32.store8 (local.get $str_loc) (local.get $lower_char))
      (local.set $str_loc  (call $incr (local.get $str_loc)))

      ;; Decrement loop variable
      (local.set $loop_offset (call $decr (local.get $loop_offset)))

      ;; Test for loop continuation
      (br_if $next_byte (i32.ge_u (local.get $loop_offset) (global.get $i64_bin_loc)))
    )
  )

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Append a line feed character to the supplied string, then write it to standard out
  ;; Input : [
  ;;   $str_loc : i32 - String location
  ;;   $str_len : i32 - String length
  ;; ]
  ;; Output : [ i32 - The number of bytes written ]
  (func $println
        (param $str_loc i32)
        (param $str_len i32)
        (result i32)

    ;; Append a line feed character to the end of the string
    ;; The string is now 1 byte longer!
    (i32.store
      (i32.add
        (local.get $str_loc)
        (local.get $str_len)
      )
      (global.get $line_feed)
    )

    ;; Store offset and (length + 1) of the data on which fd_write will operate
    (i32.store (global.get $fd_write_data_loc) (local.get $str_loc))
    (i32.store (i32.add (global.get $fd_write_data_loc) (i32.const 4)) (call $incr (local.get $str_len)))

    ;; Write the string to standard out, returning the number of bytes written
    (call $wasi_fd_write
      (i32.const 1)                   ;; fd 1 = standard out
      (global.get $fd_write_data_loc) ;; Location of string data's offset/length
      (i32.const 1)                   ;; Number of strings to write
      (i32.const 52)                  ;; fd_write stores the number of bytes written at this location
    )
  )

  ;; *******************************************************************************************************************
  ;; Public API
  ;; *******************************************************************************************************************

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Convert the raw system clock value to a character string and write it to standard out
  ;; Input  : []
  ;; Output : []
  (func (export "writeTimeNanos")
    ;; What's the time Mr WASI?
    (call $wasi_clock_time_get
      (i32.const 0)             ;; Clock id 0 = Realtime clock
      (i64.const 1)             ;; Precision
      (global.get $i64_bin_loc) ;; Write clock time to this location
    )
    drop

    ;; Convert i64 to ASCII and print
    (call $i64_to_hex_str)
    (call $println (global.get $i64_str_loc) (i32.const 16))
    drop  ;; We don't care about the value returned by println
  )

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Supply test value to function i64_to_hex_str
  ;; Input  : [i64]
  ;; Output : []
  (func (export "test_i64ToHexStr")
        (param $i64_arg i64)

    ;; Store the test value at the same location $wasi_clock_time_get writes its data
    (i64.store (global.get $i64_bin_loc) (local.get $i64_arg))

    ;; Convert i64 to ASCII and print
    (call $i64_to_hex_str)
    (call $println (global.get $i64_str_loc) (i32.const 16))
    drop  ;; We don't care about the value returned by println
  )
)
