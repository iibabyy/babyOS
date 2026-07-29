file build/isodir/boot/babyOS

define hook-stop
list
end

set substitute-path /usr/local/cargo/registry /home/ibaby/.cargo/registry
set substitute-path /workspace /home/ibaby/Desktop/babyos

set architecture i386
target remote :1234
b _entrypoint
continue