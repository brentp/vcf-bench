const std = @import("std");
const vcf = @import("./vcf.zig");
const stdout = std.io.getStdOut().writer();

pub fn main() anyerror!void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    var args = std.process.args();
    _ = args.skip();
    var fname = try args.next(&gpa.allocator).?;
    defer _ = gpa.deinit();
    defer gpa.allocator.free(fname);

    var ivcf = vcf.VCF.open(fname).?;
    defer ivcf.deinit();

    var fld: []const u8 = "AN";
    var ans = std.ArrayList(i32).init(&gpa.allocator);
    var values = std.ArrayList(i32).init(&gpa.allocator);
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

    try stdout.print("mean:{any}\n", .{s / @intToFloat(f64, values.items.len)});
}
