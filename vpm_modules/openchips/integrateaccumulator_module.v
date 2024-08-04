module integrator # (
    parameter I_BW = 5,  
    parameter O_BW = 8    
) (
    // Clock and reset
    input                               clk_i,
    input                               rst_n_i,

    // Streaming input
    input                               en_i,
    input signed [I_BW - 1 : 0]         data_i,
    input                               valid_i,

    // Streaming output
    output signed [O_BW - 1 : 0]        data_o,
    output                              valid_o
);

    // Accumulation Register
    reg signed [O_BW - 1 : 0] accumulated;

    always @(posedge clk_i) begin
        if (!rst_n_i | !en_i) begin
            accumulated <= 'd0;
        end else begin
            if (valid_i) begin
                accumulated <= accumulated + data_i;
            end
        end
    end

    // Output Assignment
    assign data_o  = accumulated;
    assign valid_o = (en_i & valid_i);

    // Simulation Only Waveform Dump (.vcd export)
    `ifdef COCOTB_SIM
    `ifndef SCANNED
    `define SCANNED
    initial begin
        $dumpfile ("wave.vcd");
        $dumpvars (0, integrator);
        #1;
    end
    `endif
    `endif

endmodule
