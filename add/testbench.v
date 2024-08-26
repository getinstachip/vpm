`timescale 1ns / 1ps

module testbench;

    // Parameters
    parameter DW = 4; // Data width

    // Inputs
    reg [DW-1:0] a;         // First operand
    reg [DW-1:0] b;         // Second operand
    reg opt_sub;            // Subtraction option
    reg cin;                // Carry in

    // Outputs
    wire [DW-1:0] sum;      // Sum output
    wire cout;              // Carry output
    wire zero;              // Zero flag
    wire neg;               // Negative flag
    wire overflow;          // Overflow flag

    // Instantiate the add module
    add #(DW) uut (
        .a(a),
        .b(b),
        .opt_sub(opt_sub),
        .cin(cin),
        .sum(sum),
        .cout(cout),
        .zero(zero),
        .neg(neg),
        .overflow(overflow)
    );

    // Test procedure
    initial begin
        // Monitor outputs
        $monitor("Time: %0t | a: %b | b: %b | opt_sub: %b | cin: %b | sum: %b | cout: %b | zero: %b | neg: %b | overflow: %b", 
                  $time, a, b, opt_sub, cin, sum, cout, zero, neg, overflow);

        // Test case 1: Simple addition
        a = 4'b0011; // 3
        b = 4'b0001; // 1
        opt_sub = 1'b0; // Addition
        cin = 1'b0; // No carry in
        #10; // Wait for 10 time units

        // Test case 2: Addition with carry
        a = 4'b0111; // 7
        b = 4'b0101; // 5
        opt_sub = 1'b0; // Addition
        cin = 1'b1; // Carry in
        #10; // Wait for 10 time units

        // Test case 3: Simple subtraction
        a = 4'b0100; // 4
        b = 4'b0011; // 3
        opt_sub = 1'b1; // Subtraction
        cin = 1'b0; // No carry in
        #10; // Wait for 10 time units

        // Test case 4: Subtraction resulting in negative
        a = 4'b0010; // 2
        b = 4'b0101; // 5
        opt_sub = 1'b1; // Subtraction
        cin = 1'b0; // No carry in
        #10; // Wait for 10 time units

        // Test case 5: Overflow condition
        a = 4'b0111; // 7
        b = 4'b0001; // 1
        opt_sub = 1'b0; // Addition
        cin = 1'b0; // No carry in
        #10; // Wait for 10 time units

        // End simulation
        $finish;
    end

endmodule