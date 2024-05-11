const std = @import("std");
const eql = std.mem.eql;

pub fn read_input(comptime file_path: []const u8, allocator: std.mem.Allocator) ![]u8 {
    return try std.fs.cwd().readFileAlloc(allocator, file_path, 25_000);
}

pub fn string_to_digit(word: []const u8) error{NotFound}!u8 {
    const wordlen = word.len;
    if (wordlen < 3) return error.NotFound;
    const numwords = comptime [9][]const u8{ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" };
    for (numwords, 0..) |numword, i| {
        const nwl = numword.len;
        if (wordlen < nwl) continue;
        for (0..(wordlen - nwl + 1)) |j|
            if (eql(u8, numword, word[j..(j + nwl)])) return @as(u8, @intCast(i + 1));
    }
    return error.NotFound;
}
