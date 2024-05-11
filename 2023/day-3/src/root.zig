const std = @import("std");

pub fn read_input(comptime file_path: []const u8, allocator: std.mem.Allocator) ![]u8 {
    return try std.fs.cwd().readFileAlloc(allocator, file_path, 25_000);
}

pub fn iter_to_vec(iterator: *std.mem.SplitIterator(u8, .sequence), allocator: std.mem.Allocator) !std.ArrayList([]const u8) {
    var list = std.ArrayList([]const u8).init(allocator);
    while (iterator.next()) |line| {
        try list.append(line);
    }
    return list;
}

pub fn is_abjacent(lines: std.ArrayList([]const u8), line_id: usize, char_id: usize) bool {
    const width = lines.items[line_id].len;
    const height = lines.items.len;

    for (0..3) |x| {
        const neighbor_x = if ((x == 0 and char_id == 0) or (x == 2 and char_id == width - 1)) continue else (char_id + x) - 1;
        for (0..3) |y| {
            if (x == 1 and y == 1) continue;
            const neighbor_y = if ((y == 0 and line_id == 0) or (y == 2 and line_id == height - 1)) continue else (line_id + y) - 1;

            const neighbor_char = lines.items[neighbor_y][neighbor_x];
            if (!std.ascii.isDigit(neighbor_char) and neighbor_char != '.') return true;
        }
    }
    return false;
}

pub const Symbol = struct { char_id: usize, line_id: usize, char: u8 };

/// return the abjacent symbol or 0 (NUL) if not found
pub fn abjacent_symbol(lines: std.ArrayList([]const u8), line_id: usize, char_id: usize) ?Symbol {
    const width = lines.items[line_id].len;
    const height = lines.items.len;

    for (0..3) |x| {
        const neighbor_x = if ((x == 0 and char_id == 0) or (x == 2 and char_id == width - 1)) continue else (char_id + x) - 1;
        for (0..3) |y| {
            if (x == 1 and y == 1) continue;
            const neighbor_y = if ((y == 0 and line_id == 0) or (y == 2 and line_id == height - 1)) continue else (line_id + y) - 1;

            const neighbor_char = lines.items[neighbor_y][neighbor_x];
            if (!std.ascii.isDigit(neighbor_char) and neighbor_char != '.') return Symbol{ .char_id = neighbor_x, .line_id = neighbor_y, .char = neighbor_char };
        }
    }
    return null;
}

pub fn pair(x: usize, y: usize) usize {
    return (x + y) * (x + y + 1) / 2 + y;
}
