const std = @import("std");
const root = @import("root.zig");

pub fn main() !void {
    // Get an allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{ .safety = true }){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    const data = try root.read_input("./input.txt", allocator);
    defer allocator.free(data);

    var rlines = std.mem.splitSequence(u8, data, "\n");
    const veclines = try root.iter_to_vec(&rlines, allocator);
    defer veclines.deinit();

    try part1(veclines, allocator);
    try part2(veclines, allocator);
}

fn part1(lines: std.ArrayList([]const u8), allocator: std.mem.Allocator) !void {
    var final_sum: usize = 0;
    for (lines.items, 0..) |line, line_id| {
        var last_non_digit: usize = 0;
        var is_abj = false;

        const exline = try std.fmt.allocPrint(allocator, "{s}.", .{line});
        defer allocator.free(exline);
        for (exline, 0..) |char, i| {
            switch (std.ascii.isDigit(char)) {
                true => is_abj = is_abj or root.is_abjacent(lines, line_id, i),
                false => {
                    if (last_non_digit == i) {
                        last_non_digit = i + 1;
                        continue;
                    }
                    const snum = line[last_non_digit..i];
                    const num = try std.fmt.parseInt(usize, snum, 10);
                    if (is_abj) final_sum += num;

                    last_non_digit = i + 1;
                    is_abj = false;
                },
            }
        }
    }

    std.debug.print("Part 1: {d}\n", .{final_sum});
}

fn part2(lines: std.ArrayList([]const u8), allocator: std.mem.Allocator) !void {
    var potential_gears = std.AutoHashMap(usize, std.ArrayList(usize)).init(allocator);
    defer potential_gears.deinit();

    for (lines.items, 0..) |line, line_id| {
        var last_non_digit: usize = 0;
        var abj_symbol: ?root.Symbol = null;

        const exline = try std.fmt.allocPrint(allocator, "{s}.", .{line});
        defer allocator.free(exline);
        for (exline, 0..) |char, i| {
            switch (std.ascii.isDigit(char)) {
                true => abj_symbol = blk: {
                    if (abj_symbol) |s| {
                        break :blk s;
                    } else {
                        break :blk root.abjacent_symbol(lines, line_id, i);
                    }
                },
                false => {
                    if (last_non_digit == i) {
                        last_non_digit = i + 1;
                        continue;
                    }
                    const snum = line[last_non_digit..i];
                    const num = try std.fmt.parseInt(usize, snum, 10);
                    if (abj_symbol) |s| {
                        if (s.char == '*') {
                            const key = root.pair(s.char_id, s.line_id);
                            if (potential_gears.getPtr(key)) |gi| {
                                try gi.append(num);
                            } else {
                                var gi = std.ArrayList(usize).init(allocator);
                                try gi.append(num);
                                try potential_gears.put(key, gi);
                            }
                        }
                    }

                    last_non_digit = i + 1;
                    abj_symbol = null;
                },
            }
        }
    }

    var result: usize = 0;
    var values = potential_gears.valueIterator();
    while (values.next()) |gi| {
        const gear_items = gi.*;
        defer gear_items.deinit();

        if (gear_items.items.len != 2) continue;
        result += gear_items.items[0] * gear_items.items[1];
    }
    std.debug.print("Part 2: {d}", .{result});
}
