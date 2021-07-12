// generated from rust-lang/libc 13c8ceb1ed9077295edf68747bb282a6bee5f31c
declare_resource! {
    #[cfg(any(
        target_os = "fuchsia",
        any(target_os = "macos", target_os = "ios"),
        any(target_os = "freebsd", target_os = "dragonfly"),
        target_os = "netbsd",
        target_os = "haiku",
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl", any(target_arch = "x86", target_arch = "mips", target_arch = "powerpc", target_arch = "hexagon", target_arch = "arm")),
        all(target_os = "linux", target_env = "musl", any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "mips64", target_arch = "powerpc64")),
        all(target_os = "linux", target_env = "uclibc", any(target_arch = "mips", target_arch = "mips64")),
        target_os = "solarish",
    ))]
    /// The maximum size (in bytes)
    /// of the process's virtual memory (address space).
    AS = 1 => RLIMIT_AS,

    #[cfg(any(
        target_os = "fuchsia",
        any(target_os = "macos", target_os = "ios"),
        any(target_os = "freebsd", target_os = "dragonfly"),
        any(target_os = "openbsd", target_os = "netbsd"),
        target_os = "haiku",
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl"),
        all(target_os = "linux", target_env = "uclibc"),
        target_os = "solarish",
    ))]
    /// The maximum size (in bytes)
    /// of a core file that the process may dump.
    CORE = 2 => RLIMIT_CORE,

    #[cfg(any(
        target_os = "fuchsia",
        any(target_os = "macos", target_os = "ios"),
        any(target_os = "freebsd", target_os = "dragonfly"),
        any(target_os = "openbsd", target_os = "netbsd"),
        target_os = "haiku",
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl"),
        all(target_os = "linux", target_env = "uclibc"),
        target_os = "solarish",
    ))]
    /// A limit (in seconds)
    /// on the amount of CPU time that the process can consume.
    CPU = 3 => RLIMIT_CPU,

    #[cfg(any(
        target_os = "fuchsia",
        any(target_os = "macos", target_os = "ios"),
        any(target_os = "freebsd", target_os = "dragonfly"),
        any(target_os = "openbsd", target_os = "netbsd"),
        target_os = "haiku",
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl"),
        all(target_os = "linux", target_env = "uclibc"),
        target_os = "solarish",
    ))]
    /// The maximum size (in bytes)
    /// of the process's data segment
    /// (initialized data, uninitialized data, and heap).
    DATA = 4 => RLIMIT_DATA,

    #[cfg(any(
        target_os = "fuchsia",
        any(target_os = "macos", target_os = "ios"),
        any(target_os = "freebsd", target_os = "dragonfly"),
        any(target_os = "openbsd", target_os = "netbsd"),
        target_os = "haiku",
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl"),
        all(target_os = "linux", target_env = "uclibc"),
        target_os = "solarish",
    ))]
    /// The maximum size (in bytes)
    /// of files that the process may create.
    FSIZE = 5 => RLIMIT_FSIZE,

    #[cfg(any(
        target_os = "freebsd",
    ))]
    /// The maximum number of kqueues this user id is allowed to create.
    KQUEUES = 6 => RLIMIT_KQUEUES,

    #[cfg(any(
        target_os = "fuchsia",
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl"),
        all(target_os = "linux", target_env = "uclibc"),
    ))]
    /// (early Linux 2.4 only)
    ///
    /// A limit on the combined number
    /// of `flock(2)` locks and `fcntl(2)` leases
    /// that this process may establish.
    LOCKS = 7 => RLIMIT_LOCKS,

    #[cfg(any(
        target_os = "fuchsia",
        any(target_os = "macos", target_os = "ios"),
        any(target_os = "freebsd", target_os = "dragonfly"),
        any(target_os = "openbsd", target_os = "netbsd"),
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl", any(target_arch = "x86", target_arch = "mips", target_arch = "powerpc", target_arch = "hexagon", target_arch = "arm")),
        all(target_os = "linux", target_env = "musl", any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "mips64", target_arch = "powerpc64")),
        all(target_os = "linux", target_env = "uclibc", any(target_arch = "mips", target_arch = "mips64")),
    ))]
    /// The maximum number (in bytes)
    /// of memory that may be locked into RAM.
    MEMLOCK = 8 => RLIMIT_MEMLOCK,

    #[cfg(any(
        target_os = "fuchsia",
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl"),
        all(target_os = "linux", target_env = "uclibc"),
    ))]
    /// A limit on the number
    /// of bytes that can be allocated for POSIX message queues
    /// for the real user ID of the calling process.
    MSGQUEUE = 9 => RLIMIT_MSGQUEUE,

    #[cfg(any(
        target_os = "fuchsia",
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl"),
        all(target_os = "linux", target_env = "uclibc"),
    ))]
    /// This specifies a ceiling
    /// to which the process's nice value can be raised
    /// using `setpriority(2)` or `nice(2)`.
    NICE = 10 => RLIMIT_NICE,

    #[cfg(any(
        target_os = "fuchsia",
        any(target_os = "macos", target_os = "ios"),
        any(target_os = "freebsd", target_os = "dragonfly"),
        any(target_os = "openbsd", target_os = "netbsd"),
        target_os = "haiku",
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl", any(target_arch = "x86", target_arch = "mips", target_arch = "powerpc", target_arch = "hexagon", target_arch = "arm")),
        all(target_os = "linux", target_env = "musl", any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "mips64", target_arch = "powerpc64")),
        all(target_os = "linux", target_env = "uclibc", any(target_arch = "mips", target_arch = "mips64")),
        target_os = "solarish",
    ))]
    /// This specifies a value
    /// one greater than the maximum file descriptor number
    /// that can be opened by this process.
    NOFILE = 11 => RLIMIT_NOFILE,

    #[cfg(any(
        target_os = "haiku",
    ))]
    /// The number of open vnode monitors.
    NOVMON = 12 => RLIMIT_NOVMON,

    #[cfg(any(
        target_os = "fuchsia",
        any(target_os = "macos", target_os = "ios"),
        any(target_os = "freebsd", target_os = "dragonfly"),
        any(target_os = "openbsd", target_os = "netbsd"),
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl", any(target_arch = "x86", target_arch = "mips", target_arch = "powerpc", target_arch = "hexagon", target_arch = "arm")),
        all(target_os = "linux", target_env = "musl", any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "mips64", target_arch = "powerpc64")),
        all(target_os = "linux", target_env = "uclibc", any(target_arch = "mips", target_arch = "mips64")),
    ))]
    /// A limit on the number of extant process (or, more precisely on Linux, threads)
    /// for the real user ID of the calling process.
    NPROC = 13 => RLIMIT_NPROC,

    #[cfg(any(
        target_os = "freebsd",
    ))]
    /// The maximum number of pseudo-terminals this user id is allowed to create.
    NPTS = 14 => RLIMIT_NPTS,

    #[cfg(any(
        target_os = "netbsd",
    ))]
    /// The maximum number of simultaneous threads (Lightweight
    /// Processes) for this user id.  Kernel threads and the
    /// first thread of each process are not counted against this
    /// limit.
    NTHR = 15 => RLIMIT_NTHR,

    #[cfg(any(
        target_os = "dragonfly",
    ))]
    /// The maximum number of POSIX-type advisory-mode locks available to this user.
    POSIXLOCKS = 16 => RLIMIT_POSIXLOCKS,

    #[cfg(any(
        target_os = "fuchsia",
        any(target_os = "macos", target_os = "ios"),
        any(target_os = "freebsd", target_os = "dragonfly"),
        any(target_os = "openbsd", target_os = "netbsd"),
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl", any(target_arch = "x86", target_arch = "mips", target_arch = "powerpc", target_arch = "hexagon", target_arch = "arm")),
        all(target_os = "linux", target_env = "musl", any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "mips64", target_arch = "powerpc64")),
        all(target_os = "linux", target_env = "uclibc", any(target_arch = "mips", target_arch = "mips64")),
    ))]
    /// A limit (in bytes)
    /// on the process's resident set
    /// (the number of virtual pages resident in RAM).
    RSS = 17 => RLIMIT_RSS,

    #[cfg(any(
        target_os = "fuchsia",
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl"),
        all(target_os = "linux", target_env = "uclibc"),
    ))]
    /// This specifies a ceiling on the real-time priority
    /// that may be set for this process
    /// using `sched_setscheduler(2)` and `sched_setparam(2)`.
    RTPRIO = 18 => RLIMIT_RTPRIO,

    #[cfg(any(
        target_os = "fuchsia",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl"),
    ))]
    /// A limit (in microseconds) on the amount of CPU time
    /// that a process scheduled under a real-time scheduling policy
    /// may consume without making a blocking system call.
    RTTIME = 19 => RLIMIT_RTTIME,

    #[cfg(any(
        any(target_os = "freebsd", target_os = "dragonfly"),
        target_os = "netbsd",
    ))]
    /// The maximum size (in bytes) of socket buffer usage for
    /// this user. This limits the amount of network memory, and
    /// hence the amount of mbufs, that this user may hold at any
    /// time.
    SBSIZE = 20 => RLIMIT_SBSIZE,

    #[cfg(any(
        target_os = "fuchsia",
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl"),
        all(target_os = "linux", target_env = "uclibc"),
    ))]
    /// A limit on the number
    /// of signals that may be queued
    /// for the real user ID of the calling process.
    SIGPENDING = 21 => RLIMIT_SIGPENDING,

    #[cfg(any(
        target_os = "fuchsia",
        any(target_os = "macos", target_os = "ios"),
        any(target_os = "freebsd", target_os = "dragonfly"),
        any(target_os = "openbsd", target_os = "netbsd"),
        target_os = "haiku",
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl"),
        all(target_os = "linux", target_env = "uclibc"),
        target_os = "solarish",
    ))]
    /// The maximum size (in bytes)
    /// of the process stack.
    STACK = 22 => RLIMIT_STACK,

    #[cfg(any(
        target_os = "freebsd",
    ))]
    /// The maximum size (in bytes) of the swap space that may be
    /// reserved or used by all of this user id's processes.
    SWAP = 23 => RLIMIT_SWAP,

    #[cfg(any(
        target_os = "freebsd",
    ))]
    /// The number of shared locks a given user may create simultaneously.
    UMTXP = 24 => RLIMIT_UMTXP,

    #[cfg(any(
        any(target_os = "freebsd", target_os = "dragonfly"),
        target_os = "solarish",
    ))]
    /// An alias for RLIMIT_AS. The maximum size of a process's mapped address space in bytes.
    VMEM = 25 => RLIMIT_VMEM,

}
