module alu #(parameter WIDTH = 32)(
    input [WIDTH-1:0] a, b,
    input [2:0] alu_ctrl,
    output reg [WIDTH-1:0] result,
    output zero
);
    always @(*) begin
        case (alu_ctrl)
            3'b000: result = a + b;   // ADD
            3'b001: result = a - b;   // SUB
            3'b010: result = a & b;   // AND
            3'b011: result = a | b;   // OR
            3'b100: result = a ^ b;   // XOR
            3'b101: result = a << b;  // SLL
            3'b110: result = a >> b;  // SRL
            3'b111: result = $signed(a) >>> b; // SRA
            default: result = 0;
        endcase
    end

    assign zero = (result == 0);
endmodule
