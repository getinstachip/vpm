module add #(parameter DW = 1) // data width
(
    input [DW-1:0]  a,        // first operand
    input [DW-1:0]  b,        // second operand
    input           opt_sub,   // subtraction option
    input           cin,       // carry in
    output [DW-1:0] sum,       // sum output
    output          cout,      // carry output
    output          zero,      // zero flag
    output          neg,       // negative flag
    output          overflow   // overflow flag
);

    wire [DW-1:0] b_sub;

    // Compute b based on the subtraction option
    assign b_sub = {(DW){opt_sub}} ^ b;

    // Zero-extend opt_sub and cin to match the width of a and b_sub
    assign {cout, sum} = a + b_sub + {DW{opt_sub}} + {DW{cin}};

    // Compute flags
    assign zero = (sum == 0);
    assign neg = sum[DW-1]; // Negative if the MSB is 1
    assign overflow = (a[DW-1] == b_sub[DW-1]) && (sum[DW-1] != a[DW-1]); // Overflow detection

endmodule