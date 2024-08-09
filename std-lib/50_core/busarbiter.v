module bus_arbiter (
    input wire clk,
    input wire reset,
    input wire [3:0] request, // Request lines from 4 devices
    output reg [3:0] grant    // Grant lines to 4 devices
);

    reg [1:0] current_grant; // Current grant index

    always @(posedge clk or posedge reset) begin
        if (reset) begin
            current_grant <= 0;
            grant <= 4'b0000;
        end else begin
            grant <= 4'b0000;
            if (request[current_grant]) begin
                grant[current_grant] <= 1;
            end
            current_grant <= current_grant + 1;
        end
    end

endmodule
