module pipeline_reg #(parameter WIDTH = 32)(
    input clk,
    input rst,
    input [WIDTH-1:0] din,
    output reg [WIDTH-1:0] dout
);
    always @(posedge clk or posedge rst) begin
        if (rst)
            dout <= 0;
        else
            dout <= din;
    end
endmodule
