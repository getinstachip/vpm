module custom_vector_adder #(
    parameter NUM_INPUTS = 8,
    parameter INPUT_WIDTH = 8
)(
    input wire [NUM_INPUTS*INPUT_WIDTH-1:0] inst_INPUT,
    output reg [INPUT_WIDTH-1:0] SUM_inst
);

    integer i;

    always @(*) begin
        SUM_inst = 0;
        for (i = 0; i < NUM_INPUTS; i = i + 1) begin
            SUM_inst = SUM_inst + inst_INPUT[i*INPUT_WIDTH +: INPUT_WIDTH];
        end
    end

endmodule
