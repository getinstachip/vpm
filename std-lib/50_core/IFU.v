// Instructional Fetch Unit
module ifu (
    input clk,
    input rst,
    input [31:0] pc_in,
    input [31:0] instr_mem [0:255],
    output reg [31:0] pc_out,
    output reg [31:0] instr_out
);
    always @(posedge clk or posedge rst) begin
        if (rst) begin
            pc_out <= 0;
        end else begin
            instr_out <= instr_mem[pc_in];
            pc_out <= pc_in + 4;
        end
    end
endmodule
