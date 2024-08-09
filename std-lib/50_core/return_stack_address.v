module return_address_stack (
    input wire clk,
    input wire reset,
    input wire push,               // Push signal
    input wire pop,                // Pop signal
    input wire [31:0] return_addr, // Return address to push
    output reg [31:0] top_addr     // Top address on stack
);

    // Stack parameters
    parameter STACK_SIZE = 16;
    reg [31:0] stack [0:STACK_SIZE-1];
    reg [3:0] sp; // Stack pointer

    always @(posedge clk or posedge reset) begin
        if (reset) begin
            sp <= 0;
        end else begin
            if (push && sp < STACK_SIZE) begin
                stack[sp] <= return_addr;
                sp <= sp + 1;
            end
            if (pop && sp > 0) begin
                sp <= sp - 1;
            end
        end
    end

    always @(*) begin
        if (sp > 0) begin
            top_addr = stack[sp-1];
        end else begin
            top_addr = 32'h00000000; // Default value when stack is empty
        end
    end

endmodule
