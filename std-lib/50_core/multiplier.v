module multiplier #(parameter WIDTH = 32)(
    input [WIDTH-1:0] a, b,
    output [2*WIDTH-1:0] product
);
    assign product = a * b;
endmodule
