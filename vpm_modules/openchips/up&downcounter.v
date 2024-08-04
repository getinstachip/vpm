module pipelined_up_down_counter #(parameter N = 4, STAGES = 2) (
    input clk,
    input reset,
    input up_down, // 1 for up, 0 for down
    output reg [N-1:0] q
);
    reg [N-1:0] pipeline [STAGES-1:0];

    always @(posedge clk or posedge reset) begin
        if (reset) begin
            q <= 0;
            pipeline[0] <= 0;
            integer i;
            for (i = 1; i < STAGES; i = i + 1) begin
                pipeline[i] <= 0;
            end
        end else begin
            if (up_down)
                pipeline[0] <= pipeline[0] + 1;
            else
                pipeline[0] <= pipeline[0] - 1;
                
            integer j;
            for (j = 1; j < STAGES; j = j + 1) begin
                pipeline[j] <= pipeline[j-1];
            end
            q <= pipeline[STAGES-1];
        end
    end
endmodule
