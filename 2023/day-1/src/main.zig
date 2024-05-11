const std = @import("std");
const root = @import("root.zig");

const is_part_2 = true;

pub fn main() !void {
    // Get an allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{ .safety = true }){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    const data = try root.read_input("./input.txt", allocator);
    defer allocator.free(data);

    var lines = std.mem.splitSequence(u8, data, "\n");

    var final_sum: usize = 0;
    while (lines.next()) |line| {
        var first_num: ?u8 = null;
        var last_num: ?u8 = null;
        var last_num_index: usize = 0;

        for (line, 0..) |char, i| {
            const found_num: ?u8 = blk: {
                if (std.ascii.isDigit(char)) break :blk (char - 48);
                break :blk if (is_part_2) root.string_to_digit(line[last_num_index..(i + 1)]) catch break :blk null else null;
            };
            if (found_num) |n| {
                first_num = first_num orelse n;
                last_num = n;
                last_num_index = i;
            }
        }

        // std.debug.print("{?d}{?d}\n", .{ first_num, last_num });
        final_sum += first_num.? * 10 + last_num.?;
    }

    std.debug.print("Result: {d}", .{final_sum});
}
