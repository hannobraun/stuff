const std = @import("std");
const testing = std.testing;

const ffi = @import("ffi.zig");

export fn add(a: i32, b: i32) i32 {
    const message = "Hello, world!";
    ffi.print(message.ptr, message.len);

    return a + b;
}

test "basic add functionality" {
    try testing.expect(add(3, 7) == 10);
}
