module divider #(parameter WIDTH = 32)(
    input clk,
    input rst,
    input start,
    input [WIDTH-1:0] dividend, divisor,
    output reg [WIDTH-1:0] quotient, remainder,
    output reg ready
);
    always @(posedge clk or posedge rst) begin
        if (rst) begin
            quotient <= 0;
            remainder <= 0;
            ready <= 0;
        end else if (start) begin
            quotient <= dividend / divisor;
            remainder <= dividend % divisor;
            ready <= 1;
        end else begin
            ready <= 0;
        end
    end
endmodule
