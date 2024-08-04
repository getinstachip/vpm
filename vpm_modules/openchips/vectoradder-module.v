module custom_vector_adder #(
    parameter NUM_INPUTS = 8,
    parameter INPUT_WIDTH = 8
)(
    input wire [NUM_INPUTS*INPUT_WIDTH-1:0] inst_INPUT,
    output wire [INPUT_WIDTH-1:0] SUM_inst
);

    // Internal signal to hold the sum
    reg [INPUT_WIDTH-1:0] sum;


    integer i;
    always @(*) begin
        sum = 0;
        for (i = 0; i < NUM_INPUTS; i = i + 1) begin
            sum = sum + inst_INPUT[i*INPUT_WIDTH +: INPUT_WIDTH];
        end
    end

    assign SUM_inst = sum;

endmodule
