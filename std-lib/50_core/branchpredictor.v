module branch_predictor (
    input clk,
    input rst,
    input branch_taken,
    output reg prediction
);
    reg state;

    always @(posedge clk or posedge rst) begin
        if (rst) begin
            state <= 0;
            prediction <= 0;
        end else begin
            prediction <= state;
            if (branch_taken)
                state <= 1;
            else
                state <= 0;
        end
    end
endmodule
