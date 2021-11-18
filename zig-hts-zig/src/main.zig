const std = @import("std");
const vcf = @import("./vcf.zig");
const stdout = std.io.getStdOut().writer();
const allocator = std.heap.c_allocator;
//const allocator = std.testing.allocator;

pub fn main() anyerror!void {
    const fname = std.mem.sliceTo(std.os.argv[1], 0);

    var ivcf = vcf.VCF.open(fname).?;
    defer ivcf.deinit();

    var fld: []const u8 = "AN";
    var ans = std.ArrayList(i32).init(allocator);
    var values = std.ArrayList(i32).init(allocator);
    defer values.deinit();
    defer ans.deinit();

    while (ivcf.next()) |*v| {
        v.get(vcf.Field.info, i32, &ans, fld) catch {
            continue;
        };
        try values.append(ans.items[0]);
    }
    var s: f64 = 0;
    for (values.items) |value| {
        s += @intToFloat(f64, value);
    }

    try stdout.print("mean:{d:4.2}\n", .{s / @intToFloat(f64, values.items.len)});
}
