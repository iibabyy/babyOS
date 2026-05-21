file target/x86-target/debug/baby_os

define hook-stop
list
end

set substitute-path /usr/local/cargo/registry /Users/ibaby/.cargo/registry
set substitute-path /workspace /Users/ibaby/projects/babyOS

set architecture i386
target remote : 1234
b _entrypoint
continue