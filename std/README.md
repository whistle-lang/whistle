

# builtins

## Core (TODO)
### definitely not wasi
- [x] `args_get(argvOffset: i32, argvBufferOffset: i32) -> none`
- [x] `args_sizes_get(argcOffset: i32, argvBufferSizeOffset: i32) -> i32 ` 
- [x] `environ_get(environOffset: i32, environBufferOffset: i32) -> none`
- [x] `environ_sizes_get(environcOffset: i32, environBufferSizeOffset: i32) -> i32`
- [x] `clock_res_get(id: i32, resolutionOffset: i32) -> i32`
- [ ] `clock_time_get(id: i32, precision: i64, timeOffset: i32) -> i32`
- [ ] `fd_advise(fd: i32, offset: i64, length: i64, advice: i32) -> i32`
- [ ] `fd_allocate(fd: i32, offset: i64, length: i64) -> i32`
- [x] `fd_close(fd: i32) -> none`
- [x] `fd_datasync(fd: i32) -> none`
- [x] `fd_fdstat_get(fd: i32, offset: i32) -> i32`
- [x] `fd_fdstat_set_flags(fd: i32, flags: i32) -> i32`
- [ ] `fd_fdstat_set_rights(fd: i32, rightsBase: i64, rightsInheriting: i64) -> i32`
- [ ] `fd_filestat_get(fd: i32, offset: i32) -> i32`
- [ ] `fd_filestat_set_size(fd: i32, size: i64) -> i32`
- [ ] `fd_filestat_set_times(fd: i32, atim: i64, mtim: i64, flags: i32) -> i32`
- [ ] `fd_pread(fd: i32, iovsOffset: i32, iovsLength: i32, offset: i64, nreadOffset: i32) -> i32`
- [ ] `fd_prestat_get(fd: i32, prestatOffset: i32) -> i32`
- [ ] `fd_prestat_dir_name(fd: i32, pathOffset: i32, pathLength: i32) -> i32`
- [ ] `fd_pwrite(fd: i32, iovsOffset: i32, iovsLength: i32, offset: i64, nwrittenOffset: i32) -> i32`
- [ ] `fd_read(fd: i32, iovsOffset: i32, iovsLength: i32, nreadOffset: i32) -> i32`
- [ ] `fd_readdir(fd: i32, bufferOffset: i32, bufferLength: i32, cookie: i64, bufferUsedOffset: i32) -> i32`
- [ ] `fd_renumber(fd: i32, to: i32) -> i32`
- [ ] `fd_seek(fd: i32, offset: i64, whence: i32, newOffsetOffset: i32) -> i32`
- [x] `fd_sync(fd: i32) -> none`
- [ ] `fd_tell(fd: i32, offsetOffset: i32) -> i32`
- [ ] `fd_write(fd: i32, iovsOffset: i32, iovsLength: i32, nwrittenOffset: i32) -> i32`
- [ ] `path_create_directory(fd: i32, pathOffset: i32, pathLength: i32) -> i32`
- [ ] `path_filestat_get(fd: i32, flags: i32, pathOffset: i32, pathLength: i32, bufferOffset: i32) -> i32`
- [ ] `path_filestat_set_times(fd: i32, flags: i32, pathOffset: i32, pathLength: i32, atim: i64, mtim: i64, fstflags: i32) -> i32`
- [ ] `path_link(oldFd: i32, oldFlags: i32, oldPathOffset: i32, oldPathLength: i32, newFd: i32, newPathOffset: i32, newPathLength: i32) -> i32`
- [ ] `path_open(fd: i32, dirflags: i32, pathOffset: i32, pathLength: i32, oflags: i32, rightsBase: i64, rightsInheriting: i64, fdFlags: i32 openedFdOffset: i32) -> i32`
- [ ] `path_readlink(fd: i32, pathOffset: i32, pathLength: i32, bufferOffset: i32, bufferLength: i32, bufferUsedOffset: i32) -> i32`
- [ ] `path_remove_directory(fd: i32, pathOffset: i32, pathLength: i32) -> i32`
- [ ] `path_rename(fd: i32, oldPathOffset: i32, oldPathLength: i32, newFd: i32, newPathOffset: i32, newPathLength: i32) -> i32`
- [ ] `path_symlink(oldPathOffset: i32, oldPathLength: i32, fd: i32, newPathOffset: i32, newPathLength: i32) -> i32`
- [ ] `path_unlink_file(fd: i32, pathOffset: i32, pathLength: i32) -> i32`
- [ ] `poll_oneoff(inOffset: i32, outOffset: i32, nsubscriptions: i32, neventsOffset: i32) -> i32`
- [x] `proc_exit(rval: i32) -> i32`
- [x] `proc_raise(sig: i32) -> i32`
- [ ] `sched_yield() -> i32`
- [ ] `random_get(bufferOffset: i32, bufferLength: i32) -> i32`
- [ ] `sock_recv(fd: i32, riDataOffset: i32, riDataLength: i32, riFlags: i32, roDataLengthOffset: i32, roFlagsOffset: i32) -> i32`