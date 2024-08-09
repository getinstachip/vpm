// Load Store Unit
module lsu #(parameter WIDTH = 32)(
    input clk,
    input rst,
    input mem_write,
    input mem_read,
    input [WIDTH-1:0] address,
    input [WIDTH-1:0] write_data,
    output reg [WIDTH-1:0] read_data,
    output reg ready
);
    reg [WIDTH-1:0] memory [0:255];

    always @(posedge clk or posedge rst) begin
        if (rst) begin
            ready <= 0;
            read_data <= 0;
        end else if (mem_write) begin
            memory[address] <= write_data;
            ready <= 1;
        end else if (mem_read) begin
            read_data <= memory[address];
            ready <= 1;
        end else begin
            ready <= 0;
        end
    end
endmodule
