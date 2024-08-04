module ripple_carry_adder #(parameter WIDTH = 4)(
    input  [WIDTH-1:0] a,  // First operand
    input  [WIDTH-1:0] b,  // Second operand
    input              cin, // Carry-in
    output [WIDTH-1:0] sum, // Sum output
    output             cout // Carry-out
);
    wire [WIDTH:0] carry;

    assign carry[0] = cin;

    genvar i;
    generate
        for (i = 0; i < WIDTH; i = i + 1) begin : adder
            full_adder FA (
                .a(a[i]),
                .b(b[i]),
                .cin(carry[i]),
                .sum(sum[i]),
                .cout(carry[i+1])
            );
        end
    endgenerate


    assign cout = carry[WIDTH];

endmodule

// Full adder module
module full_adder (
    input  a,     // First bit
    input  b,     // Second bit
    input  cin,   // Carry-in
    output sum,   // Sum output
    output cout   // Carry-out
);
    assign {cout, sum} = a + b + cin;
endmodule
