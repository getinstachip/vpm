module mac(
    input wire clk,
    input wire reset,
    input wire [7:0] a,
    input wire [7:0] b,
    input wire [15:0] acc_in,
    output reg [15:0] acc_out
);

    // Internal signals
    reg [15:0] product;
    reg [15:0] accumulator;

    always @(posedge clk or posedge reset) begin
        if (reset) begin
            accumulator <= 16'b0;
        end else begin
            product <= a * b;
            accumulator <= product + acc_in;
        end
    end

    always @(posedge clk or posedge reset) begin
        if (reset) begin
            acc_out <= 16'b0;
        end else begin
            acc_out <= accumulator;
        end
    end

endmodule
