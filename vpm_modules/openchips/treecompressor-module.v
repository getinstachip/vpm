module DW02_tree #(parameter num_inputs = 8, parameter input_width = 8, parameter verif_en = 1)
(
    input [num_inputs*input_width-1 : 0] INPUT,
    output [input_width-1 : 0] OUT0,
    output [input_width-1 : 0] OUT1
);

    // Internal signals
    reg [input_width-1 : 0] sum [0:num_inputs-1];
    reg [input_width-1 : 0] carry [0:num_inputs-1];
    integer i, j;

    always @(*) begin
        // Initialize sum and carry
        for (i = 0; i < num_inputs; i = i + 1) begin
            sum[i] = INPUT[i*input_width +: input_width];
            carry[i] = 0;
        end

        // Wallace tree reduction
        for (j = 0; j < input_width; j = j + 1) begin
            for (i = 0; i < num_inputs - 1; i = i + 3) begin
                {carry[i+1][j], sum[i][j]} = sum[i][j] + sum[i+1][j] + sum[i+2][j];
                if (i + 2 < num_inputs) begin
                    sum[i+2][j+1] = carry[i][j];
                end
            end
        end
    end

    assign OUT0 = sum[0];
    assign OUT1 = carry[0];

endmodule
