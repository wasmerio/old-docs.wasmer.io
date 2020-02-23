;; Attributions to Chris Whealy
;; https://github.com/ChrisWhealy/clock_time_get

(module
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Type declarations
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  (type $__wasi-clockTimeFnType (func (param i32 i64 i32)     (result i32)))
  (type $__wasi-fdWriteFnType   (func (param i32 i32 i32 i32) (result i32)))

  (type $unitFnType (func))

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Declare the use of native "OS" functions accessible through the WebAssembly System Interface (WASI)
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  (import "wasi_unstable" "clock_time_get" (func $wasi_unstable.clock_time_get (type $__wasi-clockTimeFnType)))
  (import "wasi_unstable" "fd_write"       (func $wasi_unstable.fd_write       (type $__wasi-fdWriteFnType)))

  ;; *******************************************************************************************************************
  ;; Private API functions
  ;; *******************************************************************************************************************

  ;; Line feed character
  (func $line-feed (result i32) i32.const 10)

  ;; Bit masks for accessing either the upper or lower nybbles of a byte
  (func $hex-mask-upper (result i32) i32.const 240)  ;; Returns 0xF0
  (func $hex-mask-lower (result i32) i32.const 15)   ;; Returns 0x0F

  ;; Increment/decrement functions
  (func $incr (param $val i32) (result i32) (i32.add (get_local $val) (i32.const 1)))
  (func $decr (param $val i32) (result i32) (i32.sub (get_local $val) (i32.const 1)))

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Convert a byte's upper or lower nybbles to the corresponding ASCII character
  ;; Input  : [ i32 ]        Integer in the range 0x0 to 0xF
  ;; Output : [ i32 ]        ASCII character of input value left on the stack
  ;;
  ;; To access a byte's upper nybble, mask out the lower nybble by AND'ing it with 0xF0
  ;; For example, if we receive input 6c
  ;;       6    C
  ;;       0110 1100
  ;;   AND 1111 0000
  ;;    -> 0110 0000
  ;;
  ;; Shift right by 4 bits to move the relevant bits to the junior half of the byte
  ;;       0000 0110
  ;;
  ;; Perform a call_indirect using 0000 0110 as the function index, thus invoking function $hex6
  ;; Function $hex6 returns the ASCII value for "6", that is, integer 54
  ;;
  ;; Similarly, to access a byte's lower nybble, mask out the upper nybble by AND'ing it with 0x0F
  ;;       6    C
  ;;       0110 1100
  ;;   AND 0000 1111
  ;;    -> 0000 1100
  ;;
  ;; No shift right is required here because the necessary bits are already in the junior half of the byte
  ;;
  ;; Perform a call_indirect using 0000 1100 as the function index, thus invoking function $hexC
  ;; Function $hexC returns the ASCII value for "c", that is, integer 99
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  (func $upper-nybble-to-char (param $upper-nybble i32) (result i32)
    (call_indirect (result i32)
      (i32.shr_u
        (i32.and
          (i32.load8_u (get_local $upper-nybble))
          (call $hex-mask-upper)
        )
        (i32.const 4)
      )
    )
  )

  (func $lower-nybble-to-char (param $lower-nybble i32) (result i32)
    (call_indirect (result i32)
      (i32.and
        (i32.load8_u (get_local $lower-nybble))
        (call $hex-mask-lower)
      )
    )
  )

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Declare a function table for hex to char conversion
  ;; The offset of each function in the table corresponds to the hex value that needs to be converted
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  (table 16 funcref)
  (elem (i32.const 0)
        $hex0 $hex1 $hex2 $hex3 $hex4 $hex5 $hex6 $hex7 $hex8 $hex9 $hexA $hexB $hexC $hexD $hexE $hexF
  )

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Function table functions
  ;; Each function returns the ASCII character corresponding to its table index
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  (func $hex0 (result i32) i32.const 48)             ;; ASCII "0"
  (func $hex1 (result i32) i32.const 49)             ;; ASCII "1"
  (func $hex2 (result i32) i32.const 50)             ;; ASCII "2"
  (func $hex3 (result i32) i32.const 51)             ;; ASCII "3"
  (func $hex4 (result i32) i32.const 52)             ;; ASCII "4"
  (func $hex5 (result i32) i32.const 53)             ;; ASCII "5"
  (func $hex6 (result i32) i32.const 54)             ;; ASCII "6"
  (func $hex7 (result i32) i32.const 55)             ;; ASCII "7"
  (func $hex8 (result i32) i32.const 56)             ;; ASCII "8"
  (func $hex9 (result i32) i32.const 57)             ;; ASCII "9"
  (func $hexA (result i32) i32.const 97)             ;; ASCII "a"
  (func $hexB (result i32) i32.const 98)             ;; ASCII "b"
  (func $hexC (result i32) i32.const 99)             ;; ASCII "c"
  (func $hexD (result i32) i32.const 100)            ;; ASCII "d"
  (func $hexE (result i32) i32.const 101)            ;; ASCII "e"
  (func $hexF (result i32) i32.const 102)            ;; ASCII "f"

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Convert a numeric binary value to an ASCII hex string.
  ;; This function assumes that the numeric binary data is stored in litte-endian format
  ;; Input        : [ $bin-offset : i32        Offset to start of binary data
  ;;                , $bin-len    : i32        Length of binary data
  ;;                , $str-offset : i32        Location at which the resulting character string will be written
  ;;                ]
  ;; Output       : [ $str-len : i32 ]         Length of generated character string
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  (func $bin-to-hex-str
        (param $bin-offset i32)
        (param $bin-len    i32)
        (param $str-offset i32)
        (result i32)

    (local $loop-offset i32)
    (local $count       i32)

    (block
      ;; Calculate offset of lowest order byte of binary data
      ;; $loop-offset = $bin-offset + $bin_len - 1
      (set_local $loop-offset (i32.add (get_local $bin-offset) (call $decr (get_local $bin-len))))

      ;; Initialise loop counter
      (set_local $count (get_local $bin-len))

      (loop
        ;; Terminate the loop if the counter has reached zero
        (br_if 1 (i32.eq (get_local $count) (i32.const 0)))

        ;; Transform the upper nybble of the current byte into text format
        ;; Write the resulting ASCII character to the offset held in $str-offset
        (i32.store8 (get_local $str-offset) (call $upper-nybble-to-char (get_local $loop-offset)))
        (set_local $str-offset (call $incr (get_local $str-offset)))

        ;; Now transform the lower nybble...
        (i32.store8 (get_local $str-offset) (call $lower-nybble-to-char (get_local $loop-offset)))
        (set_local $str-offset  (call $incr (get_local $str-offset)))

        ;; Update loop variables
        (set_local $loop-offset (call $decr (get_local $loop-offset)))
        (set_local $count       (call $decr (get_local $count)))

        ;; Restart loop
        (br 0)
      )
    )

    ;; Return the length of the generated character string
    (i32.mul (get_local $bin-len) (i32.const 2))
  )

  ;; *******************************************************************************************************************
  ;; Public API functions
  ;; *******************************************************************************************************************

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Dummy start function
  ;; If you want to call this WASM module from a JavaScript interface such as Wasmer-js, then there must be a function
  ;; called "$_start".  This function does not necessarily need to do anything, but it must be present as it will be
  ;; called automatically when the WASM instance is started
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  (func $_start (type $unitFnType))

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; What's the time Mr WASI?
  ;; Input        : []
  ;; Output       : []
  ;; Side-effects : Writes 17 bytes of ASCII time data to standard out
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  (func $getTimeNanosStr (type $unitFnType)
    (local $bin-offset i32)
    (local $bin-len    i32)
    (local $str-offset i32)
    (local $str-len    i32)

    (set_local $bin-offset (i32.const 8))
    (set_local $bin-len    (i32.const 8))
    (set_local $str-offset (i32.const 20))

    (call $wasi_unstable.clock_time_get
      (i32.const 0)           ;; Clock id 0 = Realtime clock
      (i64.const 1)           ;; Precision
      (get_local $bin-offset) ;; Offset of returned data
    )
    drop
    
    ;; Convert binary data to an ASCII hex string
    (set_local $str-len (call $bin-to-hex-str (get_local $bin-offset)
                                              (get_local $bin-len)
                                              (get_local $str-offset)
                        )
    )

    ;; Store a terminating line feed at the end of the text string ($str-offset + $str-len)
    (i32.store (i32.add (get_local $str-offset) (get_local $str-len)) (call $line-feed))

    ;; Generated character string is now one byte longer
    (set_local $str-len (i32.add (get_local $str-len) (i32.const 1)))

    ;; Store offset of string data at offset 0
    (i32.store (i32.const 0) (get_local $str-offset))

    ;; Store length of string data ($str-len) at offset 4
    (i32.store (i32.const 4) (get_local $str-len))

    ;; Write string time value to standard out
    (call $wasi_unstable.fd_write
      (i32.const 1)      ;; fd 1 = standard out
      (i32.const 0)      ;; Location of offset/length data to be written
      (i32.const 1)      ;; Number of strings to write
      (i32.const 100)    ;; Location at which the number of bytes written will be stored (not that we care...)
    )
    drop                 ;; This function returns [], so we must discard the remaining value from the top of the stack
  )

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Declare the use of one 64Kb memory page and export it using the name "memory"
  ;; If you want to call this WASM module from a JavaScript interface such as Wasmer-js, then this interface expects to
  ;; be able to access WASM memory using exactly the name "memory"
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  (memory (export "memory") 1)

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Export functions for public API
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  (export "_start"          (func $_start))
  (export "getTimeNanosStr" (func $getTimeNanosStr))
)
