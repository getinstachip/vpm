module barrel_shifter #(parameter WIDTH = 8) (
    input wire [WIDTH-1:0] data_in,  // Input data
    input wire [$clog2(WIDTH)-1:0] shift_amount, // Number of positions to shift
    input wire dir,  // Direction: 0 for left shift, 1 for right shift
    output reg [WIDTH-1:0] data_out // Output data
);

    always @(*) begin
        if (dir == 1'b0) begin
            // Left shift
            data_out = data_in << shift_amount;
        end else begin
            // Right shift
            data_out = data_in >> shift_amount;
        end
    end

endmodule
