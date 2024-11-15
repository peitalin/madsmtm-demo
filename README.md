
### System Specs

`clang++ -v`:
```
Apple clang version 15.0.0 (clang-1500.0.40.1)
Target: arm64-apple-darwin22.6.0
Thread model: posix
InstalledDir: /Library/Developer/CommandLineTools/usr/bin
```

`$ ld -v`
```
@(#)PROGRAM:ld  PROJECT:dyld-1015.6
BUILD 20:37:42 Aug 14 2023
configured to support archs: armv6 armv7 armv7s arm64 arm64e arm64_32 i386 x86_64 x86_64h armv6m armv7k armv7m armv7em
will use ld-classic for: armv6 armv7 armv7s arm64_32 i386 armv6m armv7k armv7m armv7em
LTO support using: LLVM version 15.0.0 (static support for 29, runtime is 29)
TAPI support using: Apple TAPI version 15.0.0 (tapi-1500.0.12.3)
Library search paths:
Framework search paths:
```

`sw_vers`
```
ProductName:		macOS
ProductVersion:		13.5.1
BuildVersion:		22G90
```

`$ rustc -V`
`rustc 1.82.0 (f6e511eec 2024-10-15)`


### Output
```
$ RUST_BACKTRACE=1 RUSTFLAGS="-Clinker=/usr/bin/ld" cargo run
```
```
Compiling madsmtm v0.1.0 (/Users/peita/Dev/madsmtm)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/madsmtm`
thread 'main' panicked at core/src/panicking.rs:221:5:
unsafe precondition(s) violated: ptr::replace requires that the pointer argument is aligned and non-null
stack backtrace:
   0: rust_begin_unwind
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/std/src/panicking.rs:662:5
   1: core::panicking::panic_nounwind_fmt::runtime
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/panicking.rs:112:18
   2: core::panicking::panic_nounwind_fmt
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/panicking.rs:122:5
   3: core::panicking::panic_nounwind
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/panicking.rs:221:5
   4: core::ptr::replace::precondition_check
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/ub_checks.rs:70:21
   5: core::ptr::replace
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/ub_checks.rs:77:17
   6: core::ptr::mut_ptr::<impl *mut T>::replace
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/ptr/mut_ptr.rs:1462:18
   7: std::sys::thread_local::native::lazy::Storage<T,D>::initialize
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/std/src/sys/thread_local/native/lazy.rs:66:45
   8: std::sys::thread_local::native::lazy::Storage<T,D>::get_or_init
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/std/src/sys/thread_local/native/lazy.rs:56:40
   9: madsmtm::BLOCKCHAIN_INTERFACE::{{constant}}::{{closure}}
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/std/src/sys/thread_local/native/mod.rs:99:25
  10: core::ops::function::FnOnce::call_once
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/ops/function.rs:250:5
  11: std::thread::local::LocalKey<T>::try_with
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/std/src/thread/local.rs:282:37
  12: std::thread::local::LocalKey<T>::with
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/std/src/thread/local.rs:260:9
  13: madsmtm::main
             at ./src/main.rs:16:5
  14: core::ops::function::FnOnce::call_once
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread caused non-unwinding panic. aborting.
[1]    60533 abort      RUST_BACKTRACE=1 RUSTFLAGS="-Clinker=/usr/bin/ld" cargo run
```

### rust-lldb Backtrace

`rust-lldb target/debug/madsmtm`:
```
(lldb) run
Process 61359 launched: '/Users/peita/Dev/madsmtm/target/debug/madsmtm' (arm64)
thread 'main' panicked at core/src/panicking.rs:221:5:
unsafe precondition(s) violated: ptr::replace requires that the pointer argument is aligned and non-null
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread caused non-unwinding panic. aborting.
Process 61359 stopped
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = signal SIGABRT
    frame #0: 0x000000019d244764 libsystem_kernel.dylib`__pthread_kill + 8
libsystem_kernel.dylib`:
->  0x19d244764 <+8>:  b.lo   0x19d244784               ; <+40>
    0x19d244768 <+12>: pacibsp
    0x19d24476c <+16>: stp    x29, x30, [sp, #-0x10]!
    0x19d244770 <+20>: mov    x29, sp
Target 0: (madsmtm) stopped.
(lldb) bt
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = signal SIGABRT
  * frame #0: 0x000000019d244764 libsystem_kernel.dylib`__pthread_kill + 8
    frame #1: 0x000000019d27bc28 libsystem_pthread.dylib`pthread_kill + 288
    frame #2: 0x000000019d189ae8 libsystem_c.dylib`abort + 180
    frame #3: 0x0000000100023ef4 madsmtm`std::sys::pal::unix::abort_internal::h6d5bab7b9bc17708 at mod.rs:372:14 [opt]
    frame #4: 0x00000001000226ac madsmtm`std::panicking::rust_panic_with_hook::h6a84efe4dcab239c at panicking.rs:819:9 [opt]
    frame #5: 0x0000000100021fb8 madsmtm`std::panicking::begin_panic_handler::_$u7b$$u7b$closure$u7d$$u7d$::h5eef292190467fef at panicking.rs:664:13 [opt]
    frame #6: 0x0000000100020f10 madsmtm`std::sys::backtrace::__rust_end_short_backtrace::hd7e7925203f20af9 at backtrace.rs:170:18 [opt]
    frame #7: 0x0000000100021ca8 madsmtm`rust_begin_unwind at panicking.rs:662:5 [opt]
    frame #8: 0x00000001000396cc madsmtm`core::panicking::panic_nounwind_fmt::hb2df5b6c3d6ab15b [inlined] core::panicking::panic_nounwind_fmt::runtime::hed7cc75ebcf8d970 at panicking.rs:112:18 [opt]
    frame #9: 0x00000001000396a8 madsmtm`core::panicking::panic_nounwind_fmt::hb2df5b6c3d6ab15b at panicking.rs:122:5 [opt]
    frame #10: 0x0000000100039744 madsmtm`core::panicking::panic_nounwind::h6b53a509f430e184 at panicking.rs:221:5 [opt]
    frame #11: 0x00000001000035f4 madsmtm`core::ptr::replace::precondition_check::h7757473c1bf6b11a(addr=0x0000000101008228, align=16) at ub_checks.rs:70:21
    frame #12: 0x00000001000386f8 madsmtm`std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize::hcb021295bab86086 [inlined] core::ptr::replace::h245b7a66cdd9f192(dst=0x0000000101008228, src=State<core::cell::RefCell<madsmtm::MockedBlockchain>, !> @ 0x000000016fdfb260) at ub_checks.rs:77:17
    frame #13: 0x00000001000386e4 madsmtm`std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize::hcb021295bab86086 [inlined] core::ptr::mut_ptr::_$LT$impl$u20$$BP$mut$u20$T$GT$::replace::h039fe59b959f9075(self=0x0000000101008228, src=State<core::cell::RefCell<madsmtm::MockedBlockchain>, !> @ 0x000000016fdfb260) at mut_ptr.rs:1462:18
    frame #14: 0x00000001000386e4 madsmtm`std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize::hcb021295bab86086(self=0x0000000101008228, i=Option<&mut core::option::Option<core::cell::RefCell<madsmtm::MockedBlockchain>>> @ 0x000000016fdf9490, f=0x0000000040000000) at lazy.rs:66:45
    frame #15: 0x0000000100003758 madsmtm`std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init::h62ce50527a3d4c31(self=0x0000000101008228, i=Option<&mut core::option::Option<core::cell::RefCell<madsmtm::MockedBlockchain>>> @ 0x000000016fdfe4c8, f=0x0000010100822800) at lazy.rs:56:40
    frame #16: 0x0000000100003ab8 madsmtm`madsmtm::BLOCKCHAIN_INTERFACE::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h113064b72045cee3((null)=0x000000016fdfe547, init=Option<&mut core::option::Option<core::cell::RefCell<madsmtm::MockedBlockchain>>> @ 0x000000016fdfe518) at mod.rs:99:25
    frame #17: 0x0000000100003c44 madsmtm`core::ops::function::FnOnce::call_once::h02e58a450b5fd899((null)={closure_env#1} @ 0x000000016fdfe547, (null)=(core::option::Option<&mut core::option::Option<core::cell::RefCell<madsmtm::MockedBlockchain>>>) @ 0x000000016fdfe548) at function.rs:250:5
    frame #18: 0x00000001000042ac madsmtm`std::thread::local::LocalKey$LT$T$GT$::try_with::h8eb2b96e39381607(self=0x00000001000480a8, f={closure_env#0} @ 0x000000016fdfe5b7) at local.rs:282:37
    frame #19: 0x000000010000425c madsmtm`std::thread::local::LocalKey$LT$T$GT$::with::hf14cd3a7556b48b6(self=0x00000001000480a8, f={closure_env#0} @ 0x000000016fdfe60f) at local.rs:260:9
    frame #20: 0x0000000100003e34 madsmtm`madsmtm::main::h5009ad5bc7ccf471 at main.rs:16:5
    frame #21: 0x0000000100003cc8 madsmtm`core::ops::function::FnOnce::call_once::h61a9a76ab4958805((null)=(madsmtm`madsmtm::main::h5009ad5bc7ccf471 at main.rs:15), (null)=<unavailable>) at function.rs:250:5
    frame #22: 0x0000000100003568 madsmtm`std::sys::backtrace::__rust_begin_short_backtrace::h0faaa74862e7dd75(f=(madsmtm`madsmtm::main::h5009ad5bc7ccf471 at main.rs:15)) at backtrace.rs:154:18
    frame #23: 0x0000000100003530 madsmtm`std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hd68db26498ed46de at rt.rs:164:18
    frame #24: 0x000000010001c878 madsmtm`std::rt::lang_start_internal::h9e88109c8deb8787 [inlined] core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnOnce$LT$A$GT$$u20$for$u20$$RF$F$GT$::call_once::hf77a1752ba39c45f at function.rs:284:13 [opt]
    frame #25: 0x000000010001c870 madsmtm`std::rt::lang_start_internal::h9e88109c8deb8787 [inlined] std::panicking::try::do_call::hf02556a6b145ecfc at panicking.rs:554:40 [opt]
    frame #26: 0x000000010001c86c madsmtm`std::rt::lang_start_internal::h9e88109c8deb8787 [inlined] std::panicking::try::h2bb23dba91be7e3b at panicking.rs:518:19 [opt]
    frame #27: 0x000000010001c86c madsmtm`std::rt::lang_start_internal::h9e88109c8deb8787 [inlined] std::panic::catch_unwind::h1844bc6507215052 at panic.rs:345:14 [opt]
    frame #28: 0x000000010001c86c madsmtm`std::rt::lang_start_internal::h9e88109c8deb8787 [inlined] std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::ha90e2c319598814e at rt.rs:143:48 [opt]
    frame #29: 0x000000010001c86c madsmtm`std::rt::lang_start_internal::h9e88109c8deb8787 [inlined] std::panicking::try::do_call::h7de69f625a47132a at panicking.rs:554:40 [opt]
    frame #30: 0x000000010001c86c madsmtm`std::rt::lang_start_internal::h9e88109c8deb8787 [inlined] std::panicking::try::h2198f44c68c232f7 at panicking.rs:518:19 [opt]
    frame #31: 0x000000010001c86c madsmtm`std::rt::lang_start_internal::h9e88109c8deb8787 [inlined] std::panic::catch_unwind::h40a34eeb64f44ac6 at panic.rs:345:14 [opt]
    frame #32: 0x000000010001c86c madsmtm`std::rt::lang_start_internal::h9e88109c8deb8787 at rt.rs:143:20 [opt]
    frame #33: 0x00000001000034fc madsmtm`std::rt::lang_start::h12d57609136c6450(main=(madsmtm`madsmtm::main::h5009ad5bc7ccf471 at main.rs:15), argc=1, argv=0x000000016fdfea48, sigpipe='\0') at rt.rs:163:17
    frame #34: 0x0000000100003f30 madsmtm`main + 36
    frame #35: 0x000000019cf23f28 dyld`start + 2236
```
